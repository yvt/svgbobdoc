//! This crate provides a procedural macro that renders
//! ASCII diagrams in doc comments as SVG images using [`svgbob`].
//!
//! [`svgbob`]: https://github.com/ivanceras/svgbob
//!
//! <img src="https://yvt.github.io/svgbobdoc/20190529-zhang_hilbert-2.png"
//!    style="border: 10px solid rgba(192, 192, 192, 0.15)">
//!
//! # Usage
//!
//! Add the following line to `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! svgbobdoc = "0.1"
//! ```
//!
//! Add the attribute `#[svgbobdoc::transform]` to the items to documentate.
//! Use `svgbob` code blocks to write ASCII diagrams.
//!
//!     #[svgbobdoc::transform]
//!     /// Some structure.
//!     ///
//!     /// ```svgbob
//!     ///  .--------------------.
//!     ///  | Diagrams here      |
//!     ///  `--------------------'
//!     /// ```
//!     pub struct TestStruct {}
//!
//! See the `example` directory for a complete example.
//!
//! ## Tips
//!
//!  - Using this macro increases the compilation time. If you don't mind
//!    activating unstable features, the `doc_cfg` feature ([#43781]) can be
//!    used to conditionally enable the macro by the syntax
//!    `#[cfg_attr(rustdoc, svgbobdoc::transform)]`.
//!
//! [#43781]: https://github.com/rust-lang/rust/issues/43781
extern crate proc_macro;

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input,
    spanned::Spanned,
    token, AttrStyle, Attribute, Error, Lit, LitStr, Meta, MetaNameValue, Result, Token,
};

#[derive(Clone)]
enum MaybeDocAttr {
    /// A doc comment attribute.
    ///
    /// The first `Attribute` only specifies the surround tokens.
    ///
    /// `MetaNameValue::lit` must be a `Lit::Str(_)`.
    Doc(Attribute, MetaNameValue),
    /// An unrecognized attribute that we don't care.
    Other(Attribute),
}

impl MaybeDocAttr {
    fn from_attribute(attr: Attribute) -> Result<Self> {
        if attr.path.is_ident("doc") {
            let meta = attr.parse_meta()?;

            if let Meta::NameValue(nv) = meta {
                if let Lit::Str(_) = nv.lit {
                    Ok(MaybeDocAttr::Doc(attr, nv))
                } else {
                    Err(Error::new(nv.lit.span(), "doc comment must be a string"))
                }
            } else {
                Err(Error::new(
                    meta.span(),
                    "doc comment must be a name-value attribute",
                ))
            }
        } else {
            Ok(MaybeDocAttr::Other(attr))
        }
    }
}

impl ToTokens for MaybeDocAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            MaybeDocAttr::Doc(attr, nv) => {
                attr.pound_token.to_tokens(tokens);
                if let AttrStyle::Inner(ref b) = attr.style {
                    b.to_tokens(tokens);
                }
                attr.bracket_token.surround(tokens, |tokens| {
                    nv.to_tokens(tokens);
                });
            }
            MaybeDocAttr::Other(attr) => attr.to_tokens(tokens),
        }
    }
}

/// A pre-processed brace inside an item defintion.
///
/// # Examples
///
/// ```text
/// /// foo
/// mod some_mod {
///     //! bar (this doc comment is included in `attrs`)
///     #![unrecognized_attr]
/// }
/// ```
///
/// `ts` would look like the following for the above example:
///
/// ```text
/// #![unrecognized_attr]
/// ```
///
struct ItemInner {
    /// Inner doc comments.
    attrs: Vec<MaybeDocAttr>,
    /// Everything inside the brace except the attributes extracted as `attrs`.
    ts: TokenStream,
}

impl Parse for ItemInner {
    fn parse(input: ParseStream) -> Result<Self> {
        // Extract doc comments and remove them from the token stream.
        let all_attrs = input.call(Attribute::parse_inner)?;
        let mut attrs = Vec::new();

        let mut new_tokens = TokenStream::new();

        for attr in all_attrs {
            match MaybeDocAttr::from_attribute(attr)? {
                MaybeDocAttr::Doc(attr, nv) => {
                    // Found a doc comment, move it to `attrs`.
                    // Also, turn it into an outer attribute.
                    // FIXME: I forgot why this is necessary, maybe it isn't
                    attrs.push(MaybeDocAttr::Doc(
                        Attribute {
                            style: AttrStyle::Outer,
                            ..attr
                        },
                        nv,
                    ));
                }
                MaybeDocAttr::Other(attr) => {
                    // We don't know this attribute
                    attr.to_tokens(&mut new_tokens);
                }
            }
        }

        new_tokens.extend(input.parse::<TokenStream>());

        Ok(Self {
            attrs,
            ts: new_tokens,
        })
    }
}

#[derive(Clone)]
struct Item {
    /// Inner and outer attributes, whether they are doc comments or not.
    attrs: Vec<MaybeDocAttr>,
    /// Tokens that we don't care.
    rest: TokenStream,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = input
            .call(Attribute::parse_outer)?
            .into_iter()
            .map(MaybeDocAttr::from_attribute)
            .collect::<Result<Vec<_>>>()?;

        // Look for a semicolon or an opening brace.
        let mut rest = TokenStream::new();

        while !input.peek(token::Brace) && !input.peek(Token![;]) {
            rest.extend(Some(input.parse::<TokenTree>()?));
        }

        // If an opening brace was found, look for inner attributes.
        if input.peek(token::Brace) {
            let brace: Group = input.parse()?;
            let item_inner: ItemInner = parse2(brace.stream())?;

            // Copy inner doc comments to `attrs`
            attrs.extend(item_inner.attrs);

            // Create a new `Group` without inner doc comments.
            let brace_new = Group::new(brace.delimiter(), item_inner.ts);

            rest.extend(Some(TokenTree::Group(brace_new)));
        }

        rest.extend(Some(input.parse::<TokenStream>()?));

        Ok(Self { attrs, rest })
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.rest.to_tokens(tokens);
    }
}

/// The current state of the code block finder.
#[derive(Debug)]
struct TextProcState {
    code_block: Option<CodeBlock>,
}

#[derive(Debug)]
struct CodeBlock {
    fence: String,
    captured: Option<CapturedCodeBlock>,
    start: Span,
}

#[derive(Debug)]
struct CapturedCodeBlock {
    content: String,
}

/// The output of `TextProcState::step`.
#[derive(Debug)]
enum TextProcOutput {
    /// Output the input fragment (`#[doc = "..."]`) without modification,
    /// preserving its positional information.
    Passthrough,
    /// Output nothing.
    Empty,
    /// Output a new documentation text. The positional association between the
    /// input fragment and `.0` is erased.
    Fragment(String),
}

impl TextProcState {
    fn new() -> Self {
        Self { code_block: None }
    }

    fn step(&mut self, fragment: &str, span: Span) -> TextProcOutput {
        let mut i = 0;

        let mut new_frag: Option<String> = None;

        // If `new_frag` is `None`, then this flag indicates whether the input
        // fragment is outputed as-is.
        let mut passthrough = match self.code_block {
            Some(CodeBlock {
                captured: Some(_), ..
            }) => false,
            _ => true,
        };

        // Disables "pass-through" mode, preparing `new_frag` for custom
        // generation.
        macro_rules! prepare_nonpassthrough_emission {
            () => {
                if new_frag.is_none() {
                    new_frag = Some(if passthrough {
                        fragment[0..i].to_owned()
                    } else {
                        String::new()
                    });
                }
                passthrough = false;
            };
        }

        // The use of `#[doc]` in `lazy_static!` causes name collision, so
        // wrap it with a `mod`
        mod re {
            use lazy_static::lazy_static;
            use regex::Regex;
            lazy_static! {
                pub static ref FENCE_RE: Regex =
                    Regex::new(r"^( {0,3}(?:`{3,}|~{3,}))\s*(.*?)\s*$").unwrap();
            }
        }

        fn remove_indent<'a>(mut line: &'a str, mut indent: &str) -> &'a str {
            while line.len() > 0
                && indent.len() > 0
                && line.as_bytes()[0] == indent.as_bytes()[0]
                && (indent.as_bytes()[0] == b' ' || indent.as_bytes()[0] == b'\t')
            {
                line = &line[1..];
                indent = &indent[1..];
            }
            line
        }

        loop {
            let next_break = fragment[i..].find('\n');

            let line = &fragment[i..];
            let line = if let Some(next_break) = next_break {
                &line[0..next_break]
            } else {
                line
            };

            let mut close_code_block = false;
            let mut passthrough_line = true;

            if let Some(code_block) = &mut self.code_block {
                if line == code_block.fence {
                    // Reached the end of the code block
                    if let Some(mut captured) = code_block.captured.take() {
                        passthrough_line = false;
                        prepare_nonpassthrough_emission!();

                        // Convert this captured code block to a SVG diagram.
                        captured.content.pop(); // Remove trailing "\n"
                        convert_diagram(&captured.content, new_frag.as_mut().unwrap());
                    }

                    close_code_block = true;
                } else {
                    if let Some(captured) = &mut code_block.captured {
                        captured.content += remove_indent(line, &code_block.fence);
                        captured.content.push('\n');
                        passthrough_line = false;
                    }
                }
            } else {
                // Detect a code block
                if let Some(mat) = re::FENCE_RE.captures(line) {
                    let fence = mat.get(1).unwrap().as_str();
                    let language = mat.get(2).unwrap().as_str();

                    let mut code_block = CodeBlock {
                        fence: fence.to_owned(),
                        captured: None,
                        start: span,
                    };

                    if language == "svgbob" || language.starts_with("svgbob,") {
                        // This is the code blcok we are interested in.
                        // Capture the contents.
                        passthrough_line = false;
                        code_block.captured = Some(CapturedCodeBlock {
                            content: String::new(),
                        });
                    }

                    self.code_block = Some(code_block);
                }
            }

            if close_code_block {
                self.code_block = None;
            }

            if passthrough_line {
                if let Some(new_frag) = &mut new_frag {
                    *new_frag += line;
                    if next_break.is_some() {
                        new_frag.push('\n');
                    }
                }
            } else {
                if passthrough {
                    prepare_nonpassthrough_emission!();
                }
            }

            if let Some(next_break) = next_break {
                i += next_break + 1;
            } else {
                break;
            }
        }

        if let Some(new_frag) = new_frag {
            TextProcOutput::Fragment(new_frag)
        } else if passthrough {
            TextProcOutput::Passthrough
        } else {
            TextProcOutput::Empty
        }
    }

    fn finalize(self) -> Result<()> {
        if let Some(code_block) = self.code_block {
            if code_block.captured.is_some() {
                return Err(Error::new(code_block.start, "unclosed code block"));
            }
        }
        Ok(())
    }
}

/// The font used for diagrams.
///
/// The selection made here attempts to approximate the monospace font used by
/// rustdoc's stylesheet. Source Code Pro isn't necessarily available because
/// images can't access the containing page's `@font-face`.
const DIAGRAM_FONT: &str =
    "'Source Code Pro','Andale Mono','Segoe UI Mono','Dejavu Sans Mono',monospace";

fn convert_diagram(art: &str, output: &mut String) {
    // Convert the diagram to SVG
    let mut settings = svgbob::Settings::default();
    settings.stroke_width = 1.0;
    settings.font_family = DIAGRAM_FONT.to_owned();

    let g = svgbob::Grid::from_str(art, &settings);
    let svg = g.get_svg();
    let svg_code = format!("{}", svg);

    // The use of `#[doc]` in `lazy_static!` causes name collision, so
    // wrap it with a `mod`
    mod re {
        use lazy_static::lazy_static;
        use regex::Regex;
        lazy_static! {
            pub static ref TEXT_RE: Regex = Regex::new(r"<text([^>]*)>([^<]*)</text>").unwrap();
        }
    }

    // Fix the horizontal layouting of texts by adding a `textLength` attribute
    // to `<text>` elements.
    // (The `SVG` type doesn't support traversal, and I didn't want to add
    // another dependency just for doing this.)
    let svg_code = re::TEXT_RE.replace_all(&svg_code, |captures: &regex::Captures| {
        let attr = captures.get(1).unwrap().as_str();
        let text = captures.get(2).unwrap().as_str();

        let width = width_xml_text(&text);
        let text_len = width as f32 * settings.text_width;

        format!("<text{} textLength=\"{}\">{}</text>", attr, text_len, text)
    });

    // Output the SVG as an image element
    use std::fmt::Write;
    let svg_base64 = base64::encode(&*svg_code);

    write!(output, "![](data:image/svg+xml;base64,{})", svg_base64).unwrap();
}

/// Get the EAW width of an XML-escaped string.
///
/// This function only supports entities generated by `svg_escape`.
fn width_xml_text(s: &str) -> usize {
    use unicode_width::UnicodeWidthStr;

    let mut i = 0;
    let mut width = 0;
    while let Some(k) = s[i..].find('&') {
        width += s[i..i + k].width();
        width += 1; // Characters escaped by `svg_escape` are always single-cell wide
        i += k;

        // Skip to the corresponding `;`
        i += s[i..].find(';').unwrap_or(s.len() - i - 1) + 1;
    }
    width += s[i..].width();
    width
}

/// Render ASCII-diagram code blocks in doc comments as SVG images.
///
/// See [the module-level documentation](../index.html) for more.
#[proc_macro_attribute]
pub fn transform(
    _attr: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens2 = tokens.clone();
    let mut item: Item = parse_macro_input!(tokens2);

    // Look for tagged code blocks and replace them
    let mut new_attrs = Vec::new();
    let mut text_proc = TextProcState::new();
    for attr in item.attrs.drain(..) {
        match attr {
            MaybeDocAttr::Doc(attr, mut nv) => {
                let fragment: String = if let Lit::Str(s) = &nv.lit {
                    s.value()
                } else {
                    unreachable!()
                };

                // The span used for error reporting.
                // TODO: This doesn't work somehow. Find a way to highlight the
                // very doc comment where an issue is discovered.
                let span = attr.span();

                match text_proc.step(&fragment, span) {
                    TextProcOutput::Passthrough => {
                        new_attrs.push(MaybeDocAttr::Doc(attr, nv));
                    }
                    TextProcOutput::Fragment(new_fragment) => {
                        // `nv.lit.span()` doesn't strictly apply to
                        // `new_framgent`, but we can't do better
                        let lit_str = LitStr::new(&new_fragment, nv.lit.span());
                        nv.lit = lit_str.into();
                        new_attrs.push(MaybeDocAttr::Doc(attr, nv));
                    }
                    TextProcOutput::Empty => {}
                }
            }
            MaybeDocAttr::Other(attr) => {
                new_attrs.push(MaybeDocAttr::Other(attr));
            }
        }
    }
    item.attrs = new_attrs;

    if let Err(e) = text_proc.finalize() {
        return e.to_compile_error().into();
    }

    item.into_token_stream().into()
}

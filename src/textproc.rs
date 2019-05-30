use proc_macro2::Span;
use syn::{Error, Result};

/// The current state of the code block finder.
#[derive(Debug)]
pub struct TextProcState {
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
pub enum TextProcOutput {
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
    pub fn new() -> Self {
        Self { code_block: None }
    }

    pub fn step(&mut self, fragment: &str, span: Span) -> TextProcOutput {
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

    pub fn finalize(self) -> Result<()> {
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

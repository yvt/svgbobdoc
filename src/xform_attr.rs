//! Provides the implementation of [`crate::transform()`].
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::mem::replace;
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse2, parse_macro_input,
    spanned::Spanned,
    token, AttrStyle, Attribute, DeriveInput, Lit, LitStr, Result, Token,
};

use crate::{handle_error, MaybeDocAttr};

/// A pre-processed brace inside an item defintion. Used by `OtherItem::parse`.
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
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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
struct OtherItem {
    /// Inner and outer attributes, whether they are doc comments or not.
    attrs: Vec<MaybeDocAttr>,
    /// Tokens that we don't care.
    rest: TokenStream,
}

impl Parse for OtherItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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

impl ToTokens for OtherItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.rest.to_tokens(tokens);
    }
}

/// An item processed by `transform`.
enum Item {
    Derivable(DeriveInput),
    Other(OtherItem),
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.fork().parse::<DeriveInput>().is_ok() {
            // TODO: This is not ideal from a performance point of view
            let derive_item = input.parse().unwrap();
            Ok(Item::Derivable(derive_item))
        } else {
            input.parse().map(Item::Other)
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Item::Derivable(item) => {
                item.to_tokens(tokens);
            }
            Item::Other(item) => {
                item.to_tokens(tokens);
            }
        }
    }
}

fn transform_maybedocattrs(attrs: Vec<MaybeDocAttr>) -> Result<Vec<MaybeDocAttr>> {
    use crate::textproc::{TextProcOutput, TextProcState};

    let mut new_attrs = Vec::new();
    let mut text_proc = TextProcState::new();
    for attr in attrs {
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

    text_proc.finalize()?;

    Ok(new_attrs)
}

fn transform_attributes(attrs: Vec<Attribute>) -> Result<Vec<Attribute>> {
    let mda = attrs
        .into_iter()
        .map(MaybeDocAttr::from_attribute)
        .collect::<Result<Vec<_>>>()?;

    let mda = transform_maybedocattrs(mda)?;

    Ok(mda.into_iter().map(MaybeDocAttr::into).collect())
}

fn transform_attributes_inplace(attrs: &mut Vec<Attribute>) -> Result<()> {
    *attrs = transform_attributes(replace(attrs, Vec::new()))?;
    Ok(())
}

/// The inner implementation of [`crate::transform`].
pub(super) fn transform_inner(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item: Item = parse_macro_input!(tokens);

    handle_error(|| {
        match &mut item {
            Item::Derivable(item) => {
                // The outer doc comments
                transform_attributes_inplace(&mut item.attrs)?;

                match &mut item.data {
                    syn::Data::Struct(syn::DataStruct {
                        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
                        ..
                    }) => {
                        // Process named fields
                        for field in named.iter_mut() {
                            transform_attributes_inplace(&mut field.attrs)?;
                        }
                    }
                    syn::Data::Enum(data) => {
                        // Process variants
                        for variant in data.variants.iter_mut() {
                            transform_attributes_inplace(&mut variant.attrs)?;

                            // If the variant has fields, process them as well
                            if let syn::Fields::Named(syn::FieldsNamed { named, .. }) =
                                &mut variant.fields
                            {
                                for field in named.iter_mut() {
                                    transform_attributes_inplace(&mut field.attrs)?;
                                }
                            }
                        }
                    }
                    syn::Data::Union(data) => {
                        // Process named fields
                        for field in data.fields.named.iter_mut() {
                            transform_attributes_inplace(&mut field.attrs)?;
                        }
                    }
                    _ => {}
                }
            }
            Item::Other(item) => {
                // Look for tagged code blocks and replace them
                item.attrs = transform_maybedocattrs(replace(&mut item.attrs, Vec::new()))?;
            }
        }

        Ok(item.into_token_stream().into())
    })
}

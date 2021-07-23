# svgbobdoc

[<img src="https://docs.rs/svgbobdoc/badge.svg" alt="docs.rs">](https://docs.rs/svgbobdoc/)

This crate provides a procedural macro that renders
ASCII diagrams in doc comments as SVG images using [`svgbob`].

[`svgbob`]: https://github.com/ivanceras/svgbob

<img src="https://yvt.github.io/svgbobdoc/20190529-zhang_hilbert-2.png"
   style="border: 10px solid rgba(192, 192, 192, 0.15)">

## Usage

Add the following line to `Cargo.toml`.

```toml
[dependencies]
svgbobdoc = "0.2"
```

### `transform_mdstr!`

*Requires Rust version 1.54 or later or [`extended_key_value_attributes`](https://caniuse.rs/features/extended_key_value_attrs) nightly feature.*

Wrap doc comments with `#[doc = transform_mdstr!(...)]`. Use `svgbob` code blocks to write ASCII diagrams.

    /// Some structure.
    ///
    #[doc = svgbobdoc::transform_mdstr!(
    /// ```svgbob
    ///  .--------------------.
    ///  | Diagrams here      |
    ///  `--------------------'
    /// ```
    )]
    pub struct TestStruct {}


See the `example` directory for a complete example.

### `#[transform]`

Add the attribute `#[svgbobdoc::transform]` to the items to documentate. Use `svgbob` code blocks to write ASCII diagrams.

    #[svgbobdoc::transform]
    /// Some structure.
    ///
    /// ```svgbob
    ///  .--------------------.
    ///  | Diagrams here      |
    ///  `--------------------'
    /// ```
    pub struct TestStruct {}

Limitation: This method does not work with inner attributes, meaning it's unusable for a crate-level documentation.

### Tips

 - Using this macro increases the compilation time. If you don't mind
   activating unstable features, the `doc_cfg` feature ([#43781]) can be
   used to conditionally enable the macro by the syntax
   `#[cfg_attr(doc, svgbobdoc::transform)]`.

[#43781]: https://github.com/rust-lang/rust/issues/43781

## Other forms of macros

The macro is currently implemented as an attribute macro, which has
restrictions, e.g., they cannot be attached to fields and non-inline
modules. Other forms of macros were considered, but they were unusable for
this purpose for the following reasons:

 - Function-like macros producing a part of an attribute
   (`#[svgbobdoc::doc!("...")]`): Macros in this position aren't expanded,
   causing a parsing error.

 - Function-like macros expanding to an attribute (`svgbobdoc::doc!("...")`):
   Procedural macros cannot expand to an attribute.

Therefore, despite its downsides, an attribute macro is the only working
solution known at the moment.

License: MIT/Apache-2.0

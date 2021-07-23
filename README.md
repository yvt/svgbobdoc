# svgbobdoc

[<img src="https://docs.rs/svgbobdoc/badge.svg" alt="docs.rs">](https://docs.rs/svgbobdoc/)

This crate provides a procedural macro that renders
ASCII diagrams in doc comments as SVG images using [`svgbob`].

*Requires Rust version 1.54 or later or equivalent nightly builds.*

[`svgbob`]: https://github.com/ivanceras/svgbob

<img src="https://yvt.github.io/svgbobdoc/20190529-zhang_hilbert-2.png"
   style="border: 10px solid rgba(192, 192, 192, 0.15)">

## Usage

Add the following line to `Cargo.toml`.

```toml
[dependencies]
svgbobdoc = { version = "0.2", features = ["enable"] }
```

### `transform_mdstr!`

Wrap doc comments with `#[doc = transform_mdstr!(...)]`. Use `svgbob` code blocks to write ASCII diagrams.

    #[doc = svgbobdoc::transform_mdstr!(
    /// Some structure.
    ///
    /// ```svgbob
    ///  .--------------------.
    ///  | Diagrams here      |
    ///  `--------------------'
    /// ```
    )]
    pub struct TestStruct {}


See the `example` directory for a complete example.

### `#[transform]` (deprecated)

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

 - Using this macro increases the compilation time. The `enable` Cargo feature can be used to turn off the transformation and the dependencies' compilation.

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

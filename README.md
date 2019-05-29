# svgbobdoc

[<img src="https://docs.rs/svgbobdoc/badge.svg" alt="docs.rs">](https://docs.rs/svgbobdoc/)

This crate provides a procedural macro `#[svgbobdoc::doc]` for rendering
code blocks as SVG images using [`svgbob`].

[`svgbob`]: https://github.com/ivanceras/svgbob

## Usage

Add the following line to `Cargo.toml`.

```toml
svgbobdoc = "0.1"
```

Add the attribute `#[svgbobdoc::doc]` to the items to documentate.
Use `svgbob` code blocks to write ASCII diagrams.

    #[svgbobdoc::doc]
    /// Some structure.
    ///
    /// ```svgbob,
    ///  .--------------------.
    ///  | Diagrams here      |
    ///  `--------------------'
    /// ```
    pub struct TestStruct {}

See the `example` directory for a complete example.

### Tips

 - Using this macro increases the compilation time. If you don't mind
   activating unstable features, the `doc_cfg` feature ([#43781]) can be
   used to conditionally enable the macro by the syntax
   `#[cfg_attr(rustdoc, svgbobdoc::doc)]`.

[#43781]: https://github.com/rust-lang/rust/issues/43781

License: MIT/Apache-2.0

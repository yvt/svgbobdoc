//! Some module.
//!
//! (Applying custom inner attributes currently requires
//! `#![feature(proc_macro_hygiene)]`, so the following diagram is rendered by
//! [`svgbobdoc::transform_mdstr!`] instead.)
//!
#![doc = svgbobdoc::transform_mdstr!(
//! ```svgbob,
//!  .--------------------.
//!  | Diagrams here      |
//!  `--------------------'
//! ```
)]
#![doc = svgbobdoc::transform_mdstr!("
```svgbob,
 .--------------------.
 | Diagrams here      |
 `--------------------'
```
")]

#[svgbobdoc::transform]
/// Some module.
///
/// ```svgbob,
///  .---------------.
///  | Diagrams here |
///  `---------------'
/// ```
pub mod module {
    //! ```svgbob,
    //! hoge
    //! ```
}

#[cfg_attr(doc, svgbobdoc::transform)]
/// Some function.
///
/// ```svgbob,
///  .--------------------.
///  | Diagrams here      |
///  `--------------------'
/// ```
pub fn test_function() {}

#[svgbobdoc::transform]
/// Some structure.
///
/// ```svgbob,
///  .--------------------.
///  | Diagrams here      |
///  `--------------------'
/// ```
pub struct TestStruct {
    /// Fields [can't have] attribute macros, so the struct's `#[transform]`
    /// handles the fields as well.
    ///
    /// [can't have]: https://github.com/rust-lang/rust/issues/53012
    ///
    /// ```svgbob,
    ///  .--------------------.
    ///  | Diagrams here      |
    ///  `--------------------'
    /// ```
    pub field1: u32,
}

#[svgbobdoc::transform]
/**
 * Some impl.
 *
 * ```svgbob,
 *  .--------------------.
 *  | Diagrams here      |
 *  `--------------------'
 * ```
 */
impl TestStruct {
    #[svgbobdoc::transform]
    /// Some method.
    ///
    /// ```svgbob,
    /// hoge
    /// ```
    pub fn test_method() {}
}

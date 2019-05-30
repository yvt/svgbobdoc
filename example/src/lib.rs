// These features are required to apply custom attributes on modules.
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
// The use of `#[cfg_attr(rustdoc, ...)]` is recommended to improve
// the compilation time on non-doc builds.
// Unfortunately, it's currently feature-gated.
#![feature(doc_cfg)]
// TODO: Get this working... Hopefully when they are stabilized.
// #![svgbobdoc::transform]

//! Some module.
//!
//! (The inner attributes of non-inline modules can't be processed well, so
//! the following code block isn't rendered for now.)
//!
//! ```svgbob,
//!  .--------------------.
//!  | Diagrams here      |
//!  `--------------------'
//! ```

/// Some module.
///
/// ```svgbob,
///  .---------------.
///  | Diagrams here |
///  `---------------'
/// ```
pub mod module {
    #![svgbobdoc::transform]
    //! ```svgbob,
    //! hoge
    //! ```
}

#[cfg_attr(rustdoc, svgbobdoc::transform)]
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

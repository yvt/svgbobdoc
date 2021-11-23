//! Some module.
//!
//! (Applying custom inner attributes currently requires
//! `#![feature(proc_macro_hygiene)]`, so the following diagram is rendered by
//! [`svgbobdoc::transform_mdstr!`] instead.)
//!
//! This figure is referenced by a label (`![diagram]`): ![diagram]
//!
#![doc = transform_mdstr!(
//! ```svgbob,[diagram]
//!  .----------------------.
//!  | Another diagram here |
//!  `----------------------'
//! ```
)]
#![doc = transform_mdstr!(
//! ```svgbob,
//!  .--------------------.
//!  | Diagrams here      |
//!  `--------------------'
//! ```
)]
#![doc = transform_mdstr!("
```svgbob,
 .--------------------.
 | Diagrams here      |
 `--------------------'
```
")]
use svgbobdoc::transform_mdstr;

#[doc = transform_mdstr!(
/// Some module.
///
/// ```svgbob,
///  .---------------.
///  | Diagrams here |
///  `---------------'
/// ```
)]
pub mod module {
    #![doc = transform_mdstr!(
    //! ```svgbob,
    //! hoge
    //! ```
    )]

    use svgbobdoc::transform_mdstr;
}

#[doc = transform_mdstr!(
/// Some function.
///
/// ```svgbob,
///  .--------------------.
///  | Diagrams here      |
///  `--------------------'
/// ```
)]
pub fn test_function() {}

#[doc = transform_mdstr!(
/// Some structure.
///
/// ```svgbob,
///  .--------------------.
///  | Diagrams here      |
///  `--------------------'
/// ```
)]
pub struct TestStruct {
    #[doc = transform_mdstr!(
    /// ```svgbob,
    ///  .--------------------.
    ///  | Diagrams here      |
    ///  `--------------------'
    /// ```
    )]
    pub field1: u32,
}

#[doc = transform_mdstr!(
/**
 * Some impl.
 *
 * ```svgbob,
 *  .--------------------.
 *  | Diagrams here      |
 *  `--------------------'
 * ```
 */
)]
impl TestStruct {
    #[doc = transform_mdstr!(
    /// Some method.
    ///
    /// ```svgbob,
    /// hoge
    /// ```
    )]
    pub fn test_method() {}
}

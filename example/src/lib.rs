//! Some module.
//!
//! This figure is referenced by a label (`![diagram]`): ![diagram]
//!
#![doc = transform!(
//! ```svgbob,[diagram]
//!  .----------------------.
//!  | Another diagram here |
//!  `----------------------'
//! ```
)]
#![doc = transform!(
//! ```svgbob,
//!  .----------.
//!  | Mutex<T> |
//!  `----------'
//! ```
)]
#![doc = transform!("
```svgbob,
 .--------------------.
 | Diagrams here      |
 `--------------------'
```
")]
use svgbobdoc::transform;

#[doc = transform!(
/// Some module.
///
/// ```svgbob,
///  .---------------.
///  | Diagrams here |
///  `---------------'
/// ```
)]
pub mod module {
    #![doc = transform!(
    //! ```svgbob,
    //! hoge
    //! ```
    )]

    use svgbobdoc::transform;
}

#[doc = transform!(
/// Some function.
///
/// ```svgbob,
///  .--------------------.
///  | Diagrams here      |
///  `--------------------'
/// ```
)]
pub fn test_function() {}

#[doc = transform!(
/// Some structure.
///
/// ```svgbob,
///  .--------------------.
///  | Diagrams here      |
///  `--------------------'
/// ```
)]
pub struct TestStruct {
    #[doc = transform!(
    /// ```svgbob,
    ///  .--------------------.
    ///  | Diagrams here      |
    ///  `--------------------'
    /// ```
    )]
    pub field1: u32,
}

#[doc = transform!(
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
    #[doc = transform!(
    /// Some method.
    ///
    /// ```svgbob,
    /// hoge
    /// ```
    )]
    pub fn test_method() {}
}

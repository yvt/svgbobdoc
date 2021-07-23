# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

- **Breaking** Updated `svgbob` to 0.5.
- **Breaking** Code blocks indented by more than three spaces are now processed.
- Added `transform_mdstr!`.

## [0.2.3] - 2020-10-22

- Fixed the version specification of `lazy_static`.
- Unrecognized forms of `#[doc ...]` are now ignored. Examples:
    - `#[doc(cfg(windows))` ([rust-lang/rust#43781])
    - `#[doc(include = "external-file.md")]` ([rust-lang/rust#44732])
    - `#[doc(alias = "x")]` ([rust-lang/rust#50146])

[rust-lang/rust#43781]: https://github.com/rust-lang/rust/issues/43781
[rust-lang/rust#44732]: https://github.com/rust-lang/rust/issues/44732
[rust-lang/rust#50146]: https://github.com/rust-lang/rust/issues/50146

## [0.2.2] - 2020-03-30

- Upgraded `syn`, `quote`, and `proc-macro2` to 1.x.
- Added support for `base64` 0.11 and 0.12.

## [0.2.1] - 2020-01-08

- Added a maintenance status badge

## [0.2.0] - 2019-05-30

- **Breaking** Renamed `#[svgbobdoc::doc]` to `#[svgbobdoc::transform]` because it doesn't generate `#[doc = ...]` by itself but just transforms existing `#[doc = ...]`s.
- When attached to a struct, union, or enum, `#[transform]` now transforms its fields as they cannot have attribute macros by themselves.

## 0.1.0 - 2019-05-29

- Initial release.

[Unreleased]: https://github.com/yvt/svgbobdoc/compare/0.2.3...HEAD
[0.2.3]: https://github.com/yvt/svgbobdoc/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/yvt/svgbobdoc/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/yvt/svgbobdoc/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/yvt/svgbobdoc/compare/0.1.0...0.2.0

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Upgraded `syn`, `quote`, and `proc-macro2` to 1.x.
- Added support for `base64` 0.11 and 0.12.

## [0.2.1] - 2020-01-08

- Added a maintenance status badge

## [0.2.0] - 2019-05-30

- **Breaking** Renamed `#[svgbobdoc::doc]` to `#[svgbobdoc::transform]` because it doesn't generate `#[doc = ...]` by itself but just transforms existing `#[doc = ...]`s.
- When attached to a struct, union, or enum, `#[transform]` now transforms its fields as they cannot have attribute macros by themselves.

## 0.1.0 - 2019-05-29

- Initial release.

[Unreleased]: https://github.com/yvt/svgbobdoc/compare/0.2.1...HEAD
[0.2.1]: https://github.com/yvt/svgbobdoc/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/yvt/svgbobdoc/compare/0.1.0...0.2.0

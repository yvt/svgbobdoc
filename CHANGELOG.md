# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

- **Breaking** Renamed `#[svgbobdoc::doc]` to `#[svgbobdoc::transform]` because it doesn't generate `#[doc = ...]` by itself but just transforms existing `#[doc = ...]`s. `doc!` might be added in the future.
- When attached to a struct, union, or enum, `#[transform]` now transforms its fields as they cannot have attribute macros.

## 0.1.0 - 2019-05-29

- Initial release.

[Unreleased]: https://github.com/yvt/svgbobdoc/compare/HEAD...v0.1.0

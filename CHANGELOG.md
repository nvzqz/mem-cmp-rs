# Changelog [![Crates.io][crate-badge]][crate]
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

## [0.1.4] - 2017-09-26
### Added
- `PartialOrd` blanket `impl` for `MemOrdered` works for any type that the inner
  type implements `MemOrd` for.
- Derive `Clone`, `Copy`, `Debug`, `Default`, and `Hash` for `MemOrdered`
- `#[inline]` attribute to all `pub` functions
- Slice conversion methods to `MemOrdered`

### Fixed
- Now using pointer casts instead of transmute for `MemOrdered` conversions

## [0.1.3] - 2017-08-24
### Added
- `MemOrd` impl for mixed types and slices
- `MemEq` + `MemOrd` impl for all un-sized types through specialization feature
- `MemEq` optimization using simd instructions for types of certain sizes
  - Includes avx for x86 targets

## [0.1.2] - 2017-08-22
### Fixed
- Fixed `MemOrd` impl for signed integers

## [0.1.1] - 2017-08-22
### Added
- Implemented `MemEq` for slices (`&[T]`)

## 1.0.0 - 2017-08-22

Initial release

[crate]:       https://crates.io/crates/mem_cmp
[crate-badge]: https://img.shields.io/crates/v/mem_cmp.svg

[Keep a Changelog]:    http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

[Unreleased]: https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.0...v0.1.1

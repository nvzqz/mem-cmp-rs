# Change Log

All releases of **mem_cmp** adhere to [Semantic Versioning](http://semver.org/).

---

## [v0.1.3](https://github.com/nvzqz/mem-cmp-rs/tree/v0.1.3) (2017-08-24)

- [Changes](https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.2...v0.1.3)
- [Release](https://github.com/nvzqz/mem-cmp-rs/releases/tag/v0.1.3)

### New Features
- `MemOrd` impl for mixed types and slices
- `MemEq` + `MemOrd` impl for all un-sized types through specialization feature

### Improvements
- Optimized `MemEq` to use simd instructions for types of certain sizes
    - Includes avx for x86 targets

---

## [v0.1.2](https://github.com/nvzqz/mem-cmp-rs/tree/v0.1.2) (2017-08-22)

- [Changes](https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.1...v0.1.2)
- [Release](https://github.com/nvzqz/mem-cmp-rs/releases/tag/v0.1.2)

### Fixes
- Fixed `MemOrd` impl for signed integers

---

## [v0.1.1](https://github.com/nvzqz/mem-cmp-rs/tree/v0.1.1) (2017-08-22)

- [Changes](https://github.com/nvzqz/mem-cmp-rs/compare/v0.1.0...v0.1.1)
- [Release](https://github.com/nvzqz/mem-cmp-rs/releases/tag/v0.1.1)

### New Features
- Implemented `MemEq` for slices (`&[T]`)

---

## [v1.0.0](https://github.com/nvzqz/mem-cmp-rs/tree/v1.0.0) (2017-08-22)

- [Release](https://github.com/nvzqz/mem-cmp-rs/releases/tag/v1.0.0)

Initial release

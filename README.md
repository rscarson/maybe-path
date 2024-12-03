# MaybePath
[![Crates.io](https://img.shields.io/crates/v/maybe-path.svg)](https://crates.io/crates/maybe-path)
[![Build Status](https://github.com/rscarson/maybe-path/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/rscarson/maybe-path/actions?query=branch%3Amaster)
[![docs.rs](https://img.shields.io/docsrs/maybe-path)](https://docs.rs/maybe_path/latest/maybe_path/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/maybe-path/master/LICENSE)

A Near-Zero-Overhead read-only `Path` wrapper that can also hold a `str`.  
The primary usecase is static initialization of a `Path` at compile-time.

It implements `Deref<Target = Path>`, so you can treat it as a drop-in replacement for `Path` in most cases.

Basic usage is as follows:
```rust
use maybe_path::{MaybePath, MaybePathBuf};

// These are both equivalent to `Path::new("foo/bar/baz")`
// But normally the const could not also be a `Path`
let path = MaybePath::new_path("foo/bar/baz");
const PATH: MaybePath = MaybePath::new_str("foo/bar/baz");

// An equivalent to `Cow<Path>` is also included
let not_a_cow = MaybePathBuf::new_path("foo/bar/baz");
const NOT_A_COW: MaybePathBuf = MaybePathBuf::new_str("foo/bar/baz");
```

## Performance
`MaybePath` is a zero-runtime-cost abstraction over `Path` and `str` - it does however store a u8 discriminant internally.
Benchmarks show that `MaybePath` is faster than `Cow<Path>` for most operations:  
- Read: `798.20 ps` vs `1.5002 ns`
- Clone: `811.02 ps` vs `2.3745 ns`

`MaybePathBuf`, a drop-in replacement for `Cow<Path>` that includes a 3rd state for MaybePath's `str` variant  
has performance matching that of `Cow<Path>`: ( Produced ASM is identical )
- Read: `1.5958 ns` vs `1.6596 ns`
- Clone: `3.8059 ns` vs `3.2304 ns`
- AsRef x1000: `2.1066 µs` vs `3.2081 µs`

## Safety
While it _is_ possible to access the underlying memory as-is with `as_path_unchecked` or `as_str_unchecked`,
it is not recommended to do so unless you are absolutely sure that the `MaybePath` is a `Path` or `str`.

However, in the current implementation of `Path`, all valid str's are valid paths - but this implementation detail may change in the future.

This implementation uses a union internally, since this method yields significant performance gains over using an enum.
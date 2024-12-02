A Near-Zero-Overhead read-only `Path` wrapper that can also hold a `str`.  
The primary usecase is static initialization of a `Path` at compile-time.

It implements `Deref<Target = Path>`, so you can treat it as a drop-in replacement for `Path` in most cases.

# Performance
`MaybePath` is a zero-runtime-cost abstraction over `Path` and `str`.  
Benchmarks show that `MaybePath` is faster than `Cow<Path>` for most operations:  
- Read: `798.20 ps` vs `1.5002 ns`
- Clone: `811.02 ps` vs `2.3745 ns`

However, it does store a `u8` to differentiate between `Path` and `str`,
which may increase memory usage for massive amounts of `MaybePath` instances.

# Safety
While it _is_ possible to access the underlying memory as-is with `as_path_unchecked` or `as_str_unchecked`,
it is not recommended to do so unless you are absolutely sure that the `MaybePath` is a `Path` or `str`.

However, in the current implementation of `Path`, all valid str's are valid paths - but this implementation detail may change in the future.

This implementation uses a union internally, since this method yields performance gains of up to 4x over using an enum.

```rust
use maybe_path::MaybePath;

let path = MaybePath::new_path("foo/bar/baz");
const PATH: MaybePath = MaybePath::new_str("foo/bar/baz");
```

-----

Also includes `MaybePathBuf`, a drop-in replacement for `Cow<Path>` that includes a 3rd state for MaybePath's `str` variant:

```rust
use maybe_path::MaybePathBuf;

let path = MaybePathBuf::new_path("foo/bar/baz");
const PATH: MaybePathBuf = MaybePathBuf::new_str("foo/bar/baz");
```
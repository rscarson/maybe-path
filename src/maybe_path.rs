use std::{
    ffi::OsStr,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Deref,
    path::{Path, PathBuf},
};

/// Tag to differentiate between `Path` and `str`.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum PathKind {
    Path,
    Str,
}

/// Interior storage for `MaybePath`
union InnerMaybePath<'a> {
    path: &'a Path,
    str: &'a str,
}
impl Copy for InnerMaybePath<'_> {}
impl Clone for InnerMaybePath<'_> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

/// A Near-Zero-Overhead read-only `Path` wrapper that can also hold a `str`.  
/// The primary usecase is static initialization of a `Path` at compile-time.
///
/// It implements `Deref<Target = Path>`, so you can treat it as a drop-in replacement for `Path` in most cases.
///
/// # Performance
/// `MaybePath` is a zero-runtime-cost abstraction over `Path` and `str`.  
/// Benchmarks show that `MaybePath` is faster than `Cow<Path>` for most operations:  
/// - Read: `798.20 ps` vs `1.5002 ns`
/// - Clone: `811.02 ps` vs `2.3745 ns`
///
/// However, it does store a `u8` to differentiate between `Path` and `str`,
/// which may increase memory usage for massive amounts of `MaybePath` instances.
///
/// # Safety
/// While it _is_ possible to access the underlying memory as-is with `as_path_unchecked` or `as_str_unchecked`,
/// it is not recommended to do so unless you are absolutely sure that the `MaybePath` is a `Path` or `str`.
///
/// However, in the current implementation of `Path`, all valid str's are valid paths - but this implementation detail may change in the future.
///
/// This implementation uses a union internally, since this method yields performance gains of up to 4x over using an enum.
///
/// # Examples
/// ```
/// use maybe_path::MaybePath;
///
/// let path = MaybePath::new_path("foo/bar/baz");
/// const PATH: MaybePath = MaybePath::new_str("foo/bar/baz");
/// ```
#[derive(Copy, Clone)]
pub struct MaybePath<'a> {
    kind: PathKind,
    inner: InnerMaybePath<'a>,
}

impl<'a> MaybePath<'a> {
    /// Create a new `MaybePath` from a `str`.
    /// This can be done in const-time.
    ///
    /// # Examples
    /// ```
    /// use maybe_path::MaybePath;
    /// const PATH: MaybePath = MaybePath::new_str("foo/bar/baz");
    /// ```
    pub const fn new_str(str: &'a str) -> Self {
        Self {
            kind: PathKind::Str,
            inner: InnerMaybePath { str },
        }
    }

    /// Create a new `MaybePath` from a `Path`.  
    /// Due to as-ref, this cannot be done in const-time.
    ///
    /// # Examples
    /// ```
    /// use maybe_path::MaybePath;
    /// let path = MaybePath::new_path("foo/bar/baz");
    /// ```
    pub fn new_path<P: AsRef<OsStr> + ?Sized>(path: &'a P) -> Self {
        let path = Path::new(path);
        Self {
            kind: PathKind::Path,
            inner: InnerMaybePath { path },
        }
    }

    /// Returns true if this `MaybePath` is a `Path`.  
    /// If false, `as_path` will have additional overhead
    #[inline]
    pub fn is_path(&self) -> bool {
        self.kind == PathKind::Path
    }

    /// Returns a Path reference to the underlying data.
    /// If this `MaybePath` is a `str`, this will allocate a new `Path`.
    #[inline]
    pub fn as_path(&self) -> &'a Path {
        if self.kind == PathKind::Path {
            unsafe { self.inner.path }
        } else {
            Path::new(unsafe { self.inner.str })
        }
    }

    /// Returns a str reference to the underlying data.
    /// Could return none for a `Path` if the path is not valid utf-8.
    #[inline]
    pub fn as_str(&self) -> Option<&'a str> {
        if self.kind == PathKind::Path {
            unsafe { self.inner.path.to_str() }
        } else {
            unsafe { Some(self.inner.str) }
        }
    }

    /// Returns a Path reference to the underlying data without checking if it is a `Path`.
    ///
    /// # Safety
    /// It is the caller's responsibility to ensure that this `MaybePath` is a `Path`.
    ///
    /// While the current implementation at time of writing virtually guarantees that all `str`'s are valid paths,  
    /// this is an implementation detail, and should not be relied upon.
    #[inline]
    pub unsafe fn as_path_unchecked(&self) -> &Path {
        self.inner.path
    }

    /// Returns a str reference to the underlying data without checking if it is a `str`.
    ///
    /// # Safety
    /// It is the caller's responsibility to ensure that this `MaybePath` is a `str`.
    ///
    /// While the current implementation at time of writing virtually guarantees that all `str`'s are valid paths,  
    /// The same cannot be said for paths being valid str's.
    ///
    /// It is possible for non-utf8 paths to exist, making this function unsafe.
    #[inline]
    pub unsafe fn as_str_unchecked(&self) -> &str {
        self.inner.str
    }

    /// Converts this `MaybePath` into a `PathBuf`.
    pub fn to_owned(&self) -> PathBuf {
        self.as_path().to_path_buf()
    }
}

impl Default for MaybePath<'_> {
    fn default() -> Self {
        Self::new_str("")
    }
}

impl Debug for MaybePath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("MaybePath");

        if self.is_path() {
            dbg.field("path", &self.as_path());
        } else {
            dbg.field("str", &self.as_str());
        }

        dbg.finish()
    }
}

impl Ord for MaybePath<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_path().cmp(other.as_path())
    }
}
impl PartialOrd for MaybePath<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MaybePath<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_path() == other.as_path()
    }
}
impl Eq for MaybePath<'_> {}

impl Hash for MaybePath<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.is_path() {
            self.as_path().hash(state);
        } else {
            self.as_str().hash(state);
        }
    }
}

impl Display for MaybePath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.as_path().display(), f)
    }
}

impl Deref for MaybePath<'_> {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_path()
    }
}

impl AsRef<Path> for MaybePath<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self
    }
}

impl serde::Serialize for MaybePath<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_path().serialize(serializer)
    }
}

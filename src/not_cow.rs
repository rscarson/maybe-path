use crate::MaybePath;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
};

/// A Near-Zero-Overhead read-only `Path` wrapper that can also hold a `str`, or an owned `PathBuf`.
///
/// The primary usecase is static initialization of a `Path` at compile-time.  
/// This type is designed to be a drop-in replacement for `Cow<Path>`.
///
/// Acts as a 3-state Cow<Path>:
/// - `Borrowed(&' Path)`
/// - `Borrowed(&'a str)`
/// - `Owned(PathBuf)`
///
/// # Performance
/// This type has performance matching that of `Cow<Path>`: ( Produced ASM is identical )
/// - Read: `1.5958 ns` vs `1.6596 ns`
/// - Clone: `3.8059 ns` vs `3.2304 ns`
/// - AsRef x1000: `2.1066 µs` vs `3.2081 µs`
///
/// The borrowed variant also stores a `u8` to differentiate between `Path` and `str`,
#[derive(Debug, Serialize, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum MaybePathBuf<'a> {
    /// Borrowed data
    Borrowed(MaybePath<'a>),

    /// Owned data
    Owned(PathBuf),
}

impl Default for MaybePathBuf<'_> {
    fn default() -> Self {
        Self::Borrowed(MaybePath::new_str(""))
    }
}

impl Clone for MaybePathBuf<'_> {
    #[inline]
    fn clone(&self) -> Self {
        match *self {
            Self::Borrowed(b) => Self::Borrowed(b),
            Self::Owned(ref o) => {
                let b = MaybePath::new_path(o);
                Self::Owned(b.to_owned())
            }
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (&mut Self::Owned(ref mut dest), Self::Owned(o)) => {
                MaybePath::new_path(o).as_path().clone_into(dest)
            }
            (t, s) => *t = s.clone(),
        }
    }
}

impl<'a> MaybePathBuf<'a> {
    /// Create a new `MaybePathBuf` from a `Path`.  
    /// This is the equivalent of `Cow::<Path>::Borrowed`.
    pub fn new_path<P: AsRef<OsStr> + ?Sized>(path: &'a P) -> Self {
        Self::Borrowed(MaybePath::new_path(path))
    }

    /// Create a new `MaybePathBuf` from a `str`.  
    /// This is the equivalent of `Cow::<str>::Borrowed`.
    pub const fn new_str(s: &'a str) -> Self {
        Self::Borrowed(MaybePath::new_str(s))
    }

    /// Create a new `MaybePathBuf` from a `PathBuf`.  
    /// This is the equivalent of `Cow::<Path>::Owned`.
    pub const fn new_pathbuf(path: PathBuf) -> Self {
        Self::Owned(path)
    }

    /// Returns true if the `MaybePathBuf` is borrowed.
    pub fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }

    /// Acquires a mutable reference to the owned form of the data.  
    /// If the data is borrowed, it will be cloned.
    pub fn to_mut(&mut self) -> &mut PathBuf {
        match self {
            Self::Borrowed(b) => {
                let b = b.to_owned();
                *self = Self::Owned(b);

                match self {
                    Self::Owned(ref mut o) => o,
                    _ => unreachable!(),
                }
            }

            Self::Owned(ref mut o) => o,
        }
    }

    /// Converts the `MaybePathBuf` into an owned `PathBuf`.
    pub fn into_owned(self) -> PathBuf {
        match self {
            Self::Borrowed(b) => b.to_owned(),
            Self::Owned(o) => o,
        }
    }

    /// Converts the `MaybePathBuf` into a `Cow<Path>`.
    pub fn into_cow(self) -> Cow<'a, Path> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(b.as_path()),
            Self::Owned(o) => Cow::Owned(o),
        }
    }
}

impl<'a> Borrow<Path> for MaybePathBuf<'a> {
    fn borrow(&self) -> &Path {
        self
    }
}

impl Deref for MaybePathBuf<'_> {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(b) => b.as_path(),
            Self::Owned(o) => o.as_path(),
        }
    }
}

impl AsRef<Path> for MaybePathBuf<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self
    }
}

impl<'de> Deserialize<'de> for MaybePathBuf<'de> {
    fn deserialize<D>(deserializer: D) -> Result<MaybePathBuf<'de>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        Ok(MaybePathBuf::Owned(path))
    }
}

//! General-purpose error and result types returned by public APIs of this crate.

/// The common result type returned by `citro2d` functions.
pub type Result<T> = std::result::Result<T, Error>;

/// The common error type that may be returned by `citro3d` functions.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A C2D object or context could not be initialized.
    FailedToInitialize,
    /// Failed to select the given render target for drawing to.
    InvalidRenderTarget,
    /// An invalid format parameter was specified.
    InvalidFormat,
    /// A size parameter was specified that cannot be converted to the proper type.
    InvalidSize,
    /// Attempted to use an index that was out of bounds.
    IndexOutOfBounds {
        /// The index used.
        idx: usize,
        /// The length of the collection.
        len: usize,
    },
    C3D(citro3d::Error),
}

impl From<citro3d::Error> for Error {
    fn from(value: citro3d::Error) -> Self {
        Self::C3D(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

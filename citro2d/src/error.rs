//! General-purpose error and result types returned by public APIs of this crate.

/// The common result type returned by `citro2d` functions.
pub type Result<T> = std::result::Result<T, Error>;

/// The common error type that may be returned by `citro3d` functions.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A C2D object or context could not be initialized.
    FailedToInitialize,
    /// Attempted to use an index that was out of bounds.
    IndexOutOfBounds {
        /// The index used.
        idx: usize,
        /// The length of the collection.
        len: usize,
    },
    /// Failed to select the given render target for drawing to.
    InvalidRenderTarget,
    InvalidSize,
    InvalidFormat,
}

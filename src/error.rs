use std::error::Error;

/// Contains information about which side the pointer overflow happened on.
#[derive(Debug)]
pub enum PtrOverflowBound {
    /// Returned if pointer tried to go past [isize::MAX](std::isize::MAX).
    Max,
    /// Returned if pointer tried to past 0 and into the negatives.
    Min,
}

#[derive(Debug)]
pub enum RuntimeError<IoErr: Error> {
    /// Returned if an error is encountered when reading or writing to a buffer.
    IoErr { inner: IoErr },
    /// Returned if pointer tries to go out of bounds: `0..(isize::MAX + 1)`.
    PtrOverflow { bound: PtrOverflowBound },
}

impl<IoErr: Error> From<IoErr> for RuntimeError<IoErr> {
    fn from(inner: IoErr) -> Self {
        Self::IoErr { inner }
    }
}

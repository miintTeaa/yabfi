use std::error::Error;

#[derive(Debug)]
pub enum RuntimeError<IoErr: Error> {
    /// Returned if an error is encountered when reading or writing to a buffer.
    IoErr { inner: IoErr },
    /// Returned if pointer tries to go past [isize::MAX](std::isize::MAX)
    PtrOverflow,
    /// Returned if pointer tries to go past 0 into the negatives.
    PtrUnderflow,
}

impl<IoErr: Error> From<IoErr> for RuntimeError<IoErr> {
    fn from(inner: IoErr) -> Self {
        Self::IoErr { inner }
    }
}

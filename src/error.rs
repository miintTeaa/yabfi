use std::error::Error;

#[derive(Debug)]
pub enum PtrOverflowBound {
    Max,
    Min,
}

#[derive(Debug)]
pub enum RuntimeError<IoErr: Error> {
    IoErr { inner: IoErr },
    PtrOverflow { bound: PtrOverflowBound },
}

impl<IoErr: Error> From<IoErr> for RuntimeError<IoErr> {
    fn from(inner: IoErr) -> Self {
        Self::IoErr { inner }
    }
}

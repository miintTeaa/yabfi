use std::{
    error::Error,
    fmt::{Debug, Display},
};

/// Represents a generic error when parsing and running brainfuck code.
#[derive(Debug)]
pub enum BfError<'a, IoErr: Error> {
    /// Represents a syntax error when parsing brainfuck code, see [ParseError](crate::error::ParseError).
    ParseError { inner: ParseError<'a> },
    /// Represents a runtime error when running brainfuck code, see [RuntimeError](crate::error::RuntimeError).
    RuntimeError { inner: RuntimeError<IoErr> },
}

impl<'a, IoErr: Error> Display for BfError<'a, IoErr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError { inner } => Display::fmt(inner, f),
            Self::RuntimeError { inner } => Display::fmt(inner, f),
        }
    }
}

impl<'a, IoError: Error> From<ParseError<'a>> for BfError<'a, IoError> {
    fn from(inner: ParseError<'a>) -> Self {
        BfError::ParseError { inner }
    }
}

impl<'a, IoErr: Error> From<RuntimeError<IoErr>> for BfError<'a, IoErr> {
    fn from(inner: RuntimeError<IoErr>) -> Self {
        BfError::RuntimeError { inner }
    }
}

/// Represents a syntax error when parsing brainfuck code.
#[derive(Debug)]
pub struct ParseError<'a> {
    inner: nom::error::Error<&'a str>,
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input_str: String = self.inner.input.chars().take(50).collect();
        let error_string = format!("ParseError[{:?}] at: {:?}", self.inner.code, input_str);
        f.write_str(error_string.as_str())
    }
}

impl Error for ParseError<'_> {}

impl<'a> From<nom::error::Error<&'a str>> for ParseError<'a> {
    fn from(inner: nom::error::Error<&'a str>) -> Self {
        Self { inner }
    }
}

/// Represents a runtime error when evaluating brainfuck code.
#[derive(Debug)]
pub enum RuntimeError<IoErr: Error> {
    /// Returned if an error is encountered when reading or writing to a buffer.
    IoErr { inner: IoErr },
    /// Returned if pointer tries to go past [isize::MAX](std::isize::MAX)
    PtrOverflow,
    /// Returned if pointer tries to go past 0 into the negatives.
    PtrUnderflow,
}

impl<IoErr: Error> Display for RuntimeError<IoErr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoErr { inner } => {
                f.write_str("RuntimeError[IoErr]: ")?;
                Display::fmt(inner, f)
            }
            Self::PtrOverflow => f.write_str("RuntimeError[PtrOverflow]"),
            Self::PtrUnderflow => f.write_str("RuntimeError[PtrUnderflow]"),
        }
    }
}

impl<IoErr: Error> Error for RuntimeError<IoErr> {}

impl<IoErr: Error> From<IoErr> for RuntimeError<IoErr> {
    fn from(inner: IoErr) -> Self {
        Self::IoErr { inner }
    }
}

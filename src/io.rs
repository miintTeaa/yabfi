use std::{
    error::Error,
    io::{Read, Stdin, Stdout, Write},
};

/// Allows value to be used as an output stream when in use with a [Context].
///
/// By default, this is implemented for [Stdout].
pub trait BfOutStream<Err: Error> {
    /// Writes a byte to the output stream.
    fn write(&mut self, byte: u8) -> Result<(), Err>;
}

/// Allows value to be used as an input stream when in use with a [Context].
///
/// By default, this is implemented for [Stdin].
pub trait BfInStream<Err: Error> {
    /// Reads a byte from the output stream.
    fn read(&mut self) -> Result<u8, Err>;
}

impl BfInStream<std::io::Error> for Stdin {
    fn read(&mut self) -> Result<u8, std::io::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl BfOutStream<std::io::Error> for Stdout {
    fn write(&mut self, byte: u8) -> Result<(), std::io::Error> {
        Write::write_all(self, &[byte])?;
        Ok(())
    }
}

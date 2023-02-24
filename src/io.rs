use std::{
    error::Error,
    io::{Read, Stdin, Stdout, Write},
};

pub trait BfOutStream<Err: Error> {
    fn write(&mut self, byte: u8) -> Result<(), Err>;
}

pub trait BfInStream<Err: Error> {
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

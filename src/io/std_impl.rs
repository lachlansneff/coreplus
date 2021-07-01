use crate::io::{Read, Write};

/// The bridge between [`std::io`] and [`crate::io`].
pub struct CoreIO<T>(pub T);

impl<T: std::io::Read> Read for CoreIO<T> {
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.read(buf)
    }
}

impl<T: std::io::Write> Write for CoreIO<T> {
    type Error = std::io::Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush()
    }
}

use crate::io::{Read, Write};

#[cfg(feature = "unstable")]
use crate::io::{IoSlice, IoSliceMut};

#[cfg(feature = "unstable")]
impl<'a> IoSlice<'a> for std::io::IoSlice<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self::new(buf)
    }
}

#[cfg(feature = "unstable")]
impl<'a> IoSliceMut<'a> for std::io::IoSliceMut<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Self::new(buf)
    }
}

impl<T: std::io::Read> Read for T {
    type Error = std::io::Error;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    type IoSliceMut<'a> = std::io::IoSliceMut<'a>;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf)
    }

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    fn read_vectored(&mut self, bufs: &mut [Self::IoSliceMut<'_>]) -> Result<usize, Self::Error> {
        self.read_vectored(bufs)
    }
}

impl<T: std::io::Write> Write for T {
    type Error = std::io::Error;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    type IoSlice<'a> = std::io::IoSlice<'a>;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush()
    }

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    fn write_vectored(&mut self, bufs: &[Self::IoSlice<'_>]) -> Result<usize, Self::Error> {
        self.write_vectored(bufs)
    }
}

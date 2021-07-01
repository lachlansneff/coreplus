//! I/O traits

use core::{pin::Pin, task::{Context, Poll}};

#[cfg(feature = "std")]
mod std_impl;

#[cfg(feature = "std")]
pub use self::std_impl::*;

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub trait IoSliceMut<'a>: core::ops::DerefMut<Target = [u8]> + Sized {
    fn new(buf: &'a mut [u8]) -> Self;
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub trait IoSlice<'a>: core::ops::Deref<Target = [u8]> + Sized {
    fn new(buf: &'a [u8]) -> Self;
}

/// Read bytes asynchronously.
pub trait AsyncRead {
    type Error;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    type IoSliceMut<'a>: IoSliceMut<'a>;

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>>;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [Self::IoSliceMut<'_>],
    ) -> Poll<Result<usize, Self::Error>> {
        for buf in bufs {
            if !buf.is_empty() {
                return self.poll_read(cx, buf);
            }
        }

        self.poll_read(cx, &mut [])
    }
}

/// Write bytes asynchronously.
pub trait AsyncWrite {
    type Error;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    type IoSlice<'a>: IoSlice<'a>;

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>>;

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;
    
    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[Self::IoSlice<'_>],
    ) -> Poll<Result<usize, Self::Error>> {
        for buf in bufs {
            if !buf.is_empty() {
                return self.poll_write(cx, buf);
            }
        }

        self.poll_write(cx, &[])
    }
}

/// Read bytes.
///
/// When the `std` feature is enabled (by default), this trait is automatically
/// implemented for any type that implemented [`std::io::Read`].
pub trait Read {
    type Error;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    type IoSliceMut<'a>: IoSliceMut<'a>;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    fn read_vectored(&mut self, bufs: &mut [Self::IoSliceMut<'_>]) -> Result<usize, Self::Error> {
        let mut bytes = 0;
        for buf in bufs {
            if !buf.is_empty() {
                let read = self.read(buf)?;
                bytes += read;
                if read < buf.len() {
                    break;
                }
            }
        }

        Ok(bytes)
    }
}

/// Write bytes.
///
/// When the `std` feature is enabled (by default), this trait is automatically
/// implemented for any type that implemented [`std::io::Write`].
pub trait Write {
    type Error;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    type IoSlice<'a>: IoSlice<'a>;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;

    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    fn write_vectored(&mut self, bufs: &[Self::IoSlice<'_>]) -> Result<usize, Self::Error> {
        let mut bytes = 0;
        for buf in bufs {
            let written = self.write(buf)?;
            bytes += written;
            if written < buf.len() {
                break;
            }
        }

        Ok(bytes)
    }
}

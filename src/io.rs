//! I/O traits

use core::{cmp, convert::Infallible, fmt, pin::Pin, task::{Context, Poll}, mem};

#[cfg(feature = "std")]
mod std_impl;

#[cfg(feature = "std")]
pub use self::std_impl::*;

/// Read bytes asynchronously.
pub trait AsyncRead {
    type Error;

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>>;
}

/// Write bytes asynchronously.
pub trait AsyncWrite {
    type Error;

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
}

/// Read bytes.
///
/// When the `std` feature is enabled (by default), this trait is automatically
/// implemented for any type that implemented [`std::io::Read`].
pub trait Read {
    type Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

/// Write bytes.
///
/// When the `std` feature is enabled (by default), this trait is automatically
/// implemented for any type that implemented [`std::io::Write`].
pub trait Write {
    type Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Self::Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => unreachable!("failed to write entire buffer"),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn write_fmt(&mut self, fmt: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adaptor<'a, T: Write + ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<(), T::Error>,
        }

        impl<T: Write + ?Sized> fmt::Write for Adaptor<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adaptor { inner: self, error: Ok(()) };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    unreachable!("formatter error")
                }
            }
        }
    }
}

impl Read for &[u8] {
    type Error = Infallible;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }
}

impl Write for &mut [u8] {
    type Error = Infallible;

    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        let amt = cmp::min(data.len(), self.len());
        let (a, b) = mem::replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.write(data).map(|_| {})
    }
}

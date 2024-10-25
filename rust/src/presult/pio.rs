// TODO: docs
// TODO: tests
use super::prelude::*;
use std::borrow::{Borrow, BorrowMut};
use std::convert::{AsMut, AsRef};
use std::fmt;
use std::io::{self, prelude::*, Error as IoError, ErrorKind, Result as IoRes};
use std::ops::{Deref, DerefMut};

pub type IoPRes<T> = PResult<T, IoError>;

pub trait PartialRead: Read {
    /// Read from the source and return a [`PResult`].
    fn pread(&mut self, mut buf: &mut [u8]) -> IoPRes<usize> {
        // TODO: how best to implement
        let len = buf.len();
        match io::copy(self, &mut buf) {
            Ok(n) => POk(n as usize),
            Err(e) if buf.len() == len => PErr(e),
            Err(e) => PPartial(len - buf.len(), e),
        }
    }

    fn pread_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> IoPRes<usize> {
        let buf = bufs
            .iter_mut()
            .find(|b| !b.is_empty())
            .map_or(&mut [][..], |b| &mut **b);
        self.pread(buf)
    }

    fn pread_to_end(&mut self, buf: &mut Vec<u8>) -> IoPRes<usize> {
        let len = buf.len();
        match self.read_to_end(buf) {
            Ok(n) => POk(n),
            Err(e) if buf.len() == len => PErr(e),
            Err(e) => PPartial(len - buf.len(), e),
        }
    }

    fn pread_to_string(&mut self, buf: &mut String) -> IoPRes<usize> {
        let len = buf.len();
        match self.read_to_string(buf) {
            Ok(n) => POk(n),
            Err(e) if buf.len() == len => PErr(e),
            Err(e) => PPartial(len - buf.len(), e),
        }
    }

    // TODO: read_buf, read_buf_exact

    fn pread_exact(&mut self, mut buf: &mut [u8]) -> IoPRes<usize> {
        let mut n = 0;
        while !buf.is_empty() {
            match self.pread(buf) {
                POk(0) => break,
                POk(nn) => {
                    n += nn;
                    buf = &mut buf[n..];
                }
                PPartial(nn, e) => return PPartial(n + nn, e),
                // NOTE: not exactly same implementation as io::Read::read_all
                PErr(ref e) if e.kind() == ErrorKind::Interrupted => {}
                PErr(e) => {
                    if n == 0 {
                        return PErr(e);
                    } else {
                        return PPartial(n, e);
                    }
                }
            }
        }
        POk(n)
    }
}

pub trait PartialWrite: Write {
    /// Write from the source and return a [`PResult`].
    fn pwrite(&mut self, mut buf: &[u8]) -> IoPRes<usize> {
        // TODO: how best to implement
        let len = buf.len();
        match io::copy(&mut buf, self) {
            Ok(n) => POk(n as usize),
            Err(e) if buf.len() == len => PErr(e),
            Err(e) => PPartial(len - buf.len(), e),
        }
    }

    fn pwrite_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> IoPRes<usize> {
        let buf = bufs
            .iter()
            .find(|b| !b.is_empty())
            .map_or(&[][..], |b| &**b);
        self.pwrite(buf)
    }

    fn pwrite_all(&mut self, mut buf: &[u8]) -> IoPRes<usize> {
        let mut n = 0;
        while !buf.is_empty() {
            match self.pwrite(buf) {
                POk(0) => {
                    if n == 0 {
                        return PErr(IoError::new(
                            ErrorKind::WriteZero,
                            "failed to write whole buffer",
                        ));
                    } else {
                        return PPartial(
                            n,
                            IoError::new(ErrorKind::WriteZero, "failed to write whole buffer"),
                        );
                    }
                }
                POk(nn) => {
                    n += nn;
                    buf = &buf[n..];
                }
                PPartial(nn, e) => return PPartial(n + nn, e),
                // NOTE: not exactly same implementation as io::Write::write_all
                PErr(ref e) if e.kind() == ErrorKind::Interrupted => {}
                PErr(e) => {
                    if n == 0 {
                        return PErr(e);
                    } else {
                        return PPartial(n, e);
                    }
                }
            }
        }
        POk(n)
    }

    // TODO: write_all_vectored

    fn pwrite_fmt(&mut self, fmt: fmt::Arguments<'_>) -> IoPRes<usize> {
        struct Adapter<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            res: IoPRes<usize>,
        }

        impl<T: PartialWrite + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                let res = self.inner.pwrite_all(s.as_bytes());
                self.res = res;
                if !self.res.has_err() {
                    Ok(())
                } else {
                    Err(fmt::Error)
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            res: POk(0),
        };
        match fmt::write(&mut output, fmt) {
            Ok(()) => output.res,
            Err(..) => {
                if output.res.has_err() {
                    output.res
                } else {
                    panic!(
                        "a formatting trait implementation returned an error when the underlying stream did not",
                    );
                }
            }
        }
    }
}

#[repr(transparent)]
pub struct Adapter<T>(T);

impl<T> Adapter<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub fn into_inner(Self(inner): Self) -> T {
        inner
    }
}

impl<T> Deref for Adapter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Adapter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Borrow<T> for Adapter<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> BorrowMut<T> for Adapter<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsRef<T> for Adapter<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Adapter<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Read> Read for Adapter<T> {
    fn read(&mut self, buf: &mut [u8]) -> IoRes<usize> {
        self.0.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> IoRes<usize> {
        self.0.read_vectored(bufs)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> IoRes<usize> {
        self.0.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> IoRes<usize> {
        self.0.read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> IoRes<()> {
        self.0.read_exact(buf)
    }
}

impl<T: Write> Write for Adapter<T> {
    fn write(&mut self, buf: &[u8]) -> IoRes<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> IoRes<()> {
        self.0.flush()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> IoRes<usize> {
        self.0.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> IoRes<()> {
        self.0.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> IoRes<()> {
        self.0.write_fmt(fmt)
    }
}

impl<T: Read> PartialRead for Adapter<T> {}
impl<T: Write> PartialWrite for Adapter<T> {}

pub mod prelude {
    pub use super::{PartialRead, PartialWrite};
}

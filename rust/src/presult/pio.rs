// TODO: docs
// TODO: tests
use super::prelude::*;
use std::fmt;
use std::io::{self, prelude::*, Error as IoError, ErrorKind};

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

        impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {
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

pub mod default_impls {
    use super::*;
    impl<R: Read> PartialRead for R {}
    impl<W: Write> PartialWrite for W {}
}

pub mod prelude {
    pub use super::{PartialRead, PartialWrite};
}

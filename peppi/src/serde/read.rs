use std::io::{self, Read, Result};

pub trait Read: private::Sealed {
    pub fn position(&self) -> usize;

    pub fn raw_position(&self) -> Option<usize> {
        const header_len: usize = std::mem::size_of::<crate::SLIPPI_FILE_SIGNATURE>() + 4;
        if self.pos < header_len {
            None
        } else {
            Some(self.pos - header_len)
        }
    }
}
/// input source that reads from a std::io input stream.
pub struct IoRead<R>
where
    R: io::Read,
{
    inner: R,
    pos: usize,
    last: usize,
}

impl<R> IoRead<R>
where
    R: io::Read,
{
    /// Create a replay input source to read from a std::io input stream.
    pub fn new(reader: R) -> Self {
        IoRead {
            inner: reader,
            pos: 0,
            last: 0,
        }
    }
}

impl<R: io::Read> io::Read for IoRead<R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let result = self.inner.read(buf);
		if let Ok(read) = result {
			self.pos += read;
            self.last = read;
		}
		result
	}
	fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
		self.inner
			.read_exact(buf)
			.map(|_| {
                self.pos += buf.len();
                sel.last = buf.len();
            })
	}
}

impl<R> private::Sealed for IoRead<R> where R: io::Read {}

impl<R> IoRead<R>
where
    R: io::Read,
{
    fn position(&self) -> usize {
        self.pos
    }
}


impl<R> private::Sealed for IoRead<R> where R: io::Read {}

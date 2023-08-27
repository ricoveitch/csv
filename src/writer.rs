use std::io;

pub struct Writer {
    buffer: Vec<u8>,
    delimiter: u8,
    escape: u8,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            buffer: Vec::new(),
            delimiter: b',',
            escape: b'"',
        }
    }

    fn is_special(&self, b: &u8) -> bool {
        match b {
            b'"' => true,
            b'\n' => true,
            b'\r' => true,
            b'\\' => true,
            b if b == &self.delimiter => true,
            _ => false,
        }
    }

    fn requires_escape(&self, bytes: &[u8]) -> bool {
        for b in bytes {
            if self.is_special(b) {
                return true;
            }
        }
        false
    }

    fn write_col<T: AsRef<[u8]>>(&mut self, col: T) {
        let requires_escape = self.requires_escape(col.as_ref());
        if requires_escape {
            self.buffer.push(self.escape);
        }

        for byte in col.as_ref() {
            self.buffer.push(byte.to_owned());
        }

        if requires_escape {
            self.buffer.push(self.escape);
        }
    }

    pub fn write_row<I: IntoIterator<Item = T>, T: AsRef<[u8]>>(&mut self, row: I) {
        if self.buffer.len() > 0 {
            self.buffer.push(b'\n');
        }

        let mut iter = row.into_iter();

        if let Some(col) = iter.next() {
            self.write_col(col);
        }

        for col in iter {
            self.buffer.push(self.delimiter);
            self.write_col(col);
        }
    }

    pub fn write_to_stream<W: io::Write>(&self, mut stream: W) -> io::Result<()> {
        stream.write_all(&self.buffer)?;
        Ok(())
    }

    pub fn to_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

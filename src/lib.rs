use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

type ReaderRowResult = Result<Vec<String>, Box<dyn Error>>;

struct ReaderOptions {
    delimiter: char,
    skip_headers: bool,
    trim: bool,
}

impl Default for ReaderOptions {
    fn default() -> Self {
        ReaderOptions {
            delimiter: ',',
            skip_headers: true,
            trim: true,
        }
    }
}

struct ReaderState {
    headers: Option<Vec<String>>,
    seeked: bool,
}

struct Reader<R: io::Read> {
    reader: Lines<BufReader<R>>,
    options: ReaderOptions,
    state: ReaderState,
}

impl<R: io::Read> Reader<R> {
    //    pub fn delimiter(&mut self, delimiter: String) -> &mut Reader {
    //        self.delimiter = delimiter;
    //        self
    //    }
    //
    //    pub fn skip_headers(&mut self, skip_headers: bool) -> &mut Reader {
    //        self.skip_headers = skip_headers;
    //        self
    //    }
    //
    //    pub fn parse(&self, file: File) {
    //        let reader = BufReader::new(file);
    //    }
    //
    pub fn from(read: R) -> Reader<R> {
        Reader {
            reader: BufReader::new(read).lines(),
            options: ReaderOptions::default(),
            state: ReaderState {
                headers: None,
                seeked: false,
            },
        }
    }

    pub fn iter(&mut self) -> ReaderIterator<R> {
        ReaderIterator { reader: self }
    }

    pub fn read_row(&mut self) -> Option<ReaderRowResult> {
        let result = match self.reader.next() {
            Some(r) => r,
            None => return None,
        };

        match result {
            Ok(row) => {
                let fields: Vec<String> = row
                    .trim()
                    .split(self.options.delimiter)
                    .map(|s| {
                        if self.options.trim {
                            s.trim().to_string()
                        } else {
                            s.to_string()
                        }
                    })
                    .collect();

                if !self.state.seeked && self.state.headers.is_none() {
                    self.state.seeked = true;
                    self.state.headers = Some(fields.clone());

                    if self.options.skip_headers {
                        return self.read_row();
                    }
                }

                Some(Ok(fields))
            }
            Err(_) => None,
        }
    }
}

struct ReaderIterator<'r, R: io::Read> {
    reader: &'r mut Reader<R>,
}

impl<'r, R: io::Read> Iterator for ReaderIterator<'r, R> {
    type Item = ReaderRowResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_row()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file() -> Result<(), Box<dyn Error>> {
        let file = File::open("data/empty.csv")?;
        let mut reader = Reader::from(file);

        for _ in reader.iter() {
            assert_eq!(true, false)
        }

        assert_eq!(reader.state.headers, None);

        Ok(())
    }
}

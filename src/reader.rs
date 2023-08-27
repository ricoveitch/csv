use std::{
    error::Error,
    io::{self, BufRead, BufReader, Lines},
};

type ReaderRowResult = Result<Vec<String>, Box<dyn Error>>;

struct ReaderOptions {
    delimiter: char,
    skip_headers: bool,
    trim: bool,
}

pub struct ReaderBuilder {
    delimiter: char,
    skip_headers: bool,
    trim: bool,
}

impl ReaderBuilder {
    pub fn delimiter(&mut self, delimiter: char) -> &mut ReaderBuilder {
        self.delimiter = delimiter;
        self
    }

    pub fn skip_headers(&mut self, skip_headers: bool) -> &mut ReaderBuilder {
        self.skip_headers = skip_headers;
        self
    }

    pub fn from<R: io::Read>(&self, read: R) -> Reader<R> {
        Reader::make(
            read,
            ReaderOptions {
                delimiter: self.delimiter,
                skip_headers: self.skip_headers,
                trim: self.trim,
            },
        )
    }

    pub fn new() -> ReaderBuilder {
        ReaderBuilder {
            delimiter: ',',
            skip_headers: true,
            trim: true,
        }
    }
}

pub struct ReaderState {
    pub headers: Option<Vec<String>>,
}

pub struct Reader<R: io::Read> {
    reader: Lines<BufReader<R>>,
    options: ReaderOptions,
    pub state: ReaderState,
}

impl<R: io::Read> Reader<R> {
    pub fn from(read: R) -> Reader<R> {
        Reader::make(
            read,
            ReaderOptions {
                delimiter: ',',
                skip_headers: true,
                trim: true,
            },
        )
    }

    fn make(read: R, options: ReaderOptions) -> Reader<R> {
        Reader {
            reader: BufReader::new(read).lines(),
            options,
            state: ReaderState { headers: None },
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

                if self.state.headers.is_none() {
                    self.state.headers = Some(fields.clone());

                    if self.options.skip_headers {
                        return self.read_row();
                    }
                }

                Some(Ok(fields))
            }
            Err(err) => Some(Err(Box::new(err))),
        }
    }
}

pub struct ReaderIterator<'r, R: io::Read> {
    reader: &'r mut Reader<R>,
}

impl<'r, R: io::Read> Iterator for ReaderIterator<'r, R> {
    type Item = ReaderRowResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_row()
    }
}

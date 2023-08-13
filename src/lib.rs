use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

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

struct Reader {
    reader: Lines<BufReader<File>>,
    reader_opts: ReaderOptions,
}

impl Reader {
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
    //    pub fn rows(&self) {
    //        // return an iterator
    //    }

    pub fn from(file: File) -> Reader {
        let mut reader = BufReader::new(file).lines();
        reader.next();

        Reader {
            reader,
            reader_opts: ReaderOptions::default(),
        }
    }
}

impl Iterator for Reader {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.reader.next() {
            Some(r) => r,
            None => return None,
        };

        match result {
            Ok(row) => {
                let fields: Vec<String> = row
                    .trim()
                    .split(self.reader_opts.delimiter)
                    .map(|s| {
                        if self.reader_opts.trim {
                            s.trim().to_string()
                        } else {
                            s.to_string()
                        }
                    })
                    .collect();
                Some(fields)
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let file = File::open("data/1.csv")?;
        let reader = Reader::from(file);

        for row in reader {
            println!("{:?}", row)
        }

        Ok(())
    }
}

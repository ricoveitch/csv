use csv::reader::{Reader, ReaderBuilder};
use csv::writer::Writer;
use std::{error::Error, fs::File};

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

    #[test]
    fn reader() -> Result<(), Box<dyn Error>> {
        let file = File::open("data/1.csv")?;
        let mut reader = Reader::from(file);

        for result in reader.iter() {
            let row = result?;
            assert_eq!(row.len(), 3)
        }

        assert!(reader.state.headers.is_some());
        if let Some(headers) = reader.state.headers {
            assert_eq!(headers[0], "h1");
            assert_eq!(headers[1], "h2");
            assert_eq!(headers[2], "h3");
        }

        Ok(())
    }

    #[test]
    fn builder() -> Result<(), Box<dyn Error>> {
        let file = File::open("data/sep-|.csv")?;
        let mut reader = ReaderBuilder::new().delimiter('|').from(file);

        for row in reader.iter() {
            let r = row?;
            assert_eq!(r.len(), 2);
        }
        Ok(())
    }

    fn get_string(b: &[u8]) -> Result<String, Box<dyn Error>> {
        let s = String::from_utf8(b.to_vec())?;
        Ok(s)
    }

    #[test]
    fn write_1() -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::new();
        writer.write_row(&["a", "b", "c"]);
        let contents = get_string(writer.to_buffer())?;
        assert_eq!(contents, "a,b,c");
        Ok(())
    }

    #[test]
    fn write_2() -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::new();
        writer.write_row(&["a", "", "c"]);
        writer.write_row(&["a", "\n", ","]);
        let contents = get_string(writer.to_buffer())?;
        assert_eq!(contents, "a,,c\na,\"\n\",\",\"");
        Ok(())
    }
}

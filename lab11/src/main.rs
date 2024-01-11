use ::std::fs::File;
use std::io::{Error, Write};
struct MyWriter<W:Write> {
    writer: W,
    content: Vec<u8>,
}

impl<W:Write> MyWriter<W> {
    fn new(given_write_object: W) -> MyWriter<W> {
        MyWriter {
            writer: given_write_object,
            content: Vec::new(),
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for byte in buf {
            self.content.push(*byte);
            self.content.push(*byte);
        }

        let result: &[u8] = &self.content;

        let bytes_written = self.writer.write(result)?;
        self.content.clear();

        Ok(bytes_written)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        for byte in buf {
            self.content.push(*byte);
            self.content.push(*byte);
        }

        let result: &[u8] = &self.content;

        self.writer.write_all(result)?;
        self.content.clear();
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;
    writer.write(b"def")?;

    let mut stdout_writer = MyWriter::new(std::io::stdout());
    stdout_writer.write_all(b"def")?;

    Ok(())
}

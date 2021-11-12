use anyhow::Result;
use serde::Serialize;
use serde_json::ser::{Formatter, Serializer};
use serde_json::{from_reader, Value};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

#[derive(Debug, Clone, Default)]
struct NonBreakingFormatter;

impl<'a> Formatter for NonBreakingFormatter {
    #[inline]
    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = [0; 4];
        for char in fragment.chars() {
            if char == '\u{00A0}' {
                writer.write_all(b"\\u00A0")?;
            } else {
                writer.write_all(char.encode_utf8(&mut buffer).as_bytes())?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let value: Value = from_reader(reader)?;
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let mut ser = Serializer::with_formatter(writer, NonBreakingFormatter);
        value.serialize(&mut ser)?;
    }
    Ok(())
}

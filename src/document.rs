use std::fs::File;
use std::io::BufRead;
use std::io::{BufReader, Result};

#[derive(Default)]
pub struct Document {
    pub file_path: String,
    pub lines: Vec<String>,
}

impl Document {
    pub fn open(file_path: &str) -> Result<Document> {
        let file = File::open(file_path)?;
        let lines = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>>>()?;
        Ok(Document {
            file_path: file_path.into(),
            lines,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}

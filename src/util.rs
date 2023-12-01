use std::fs::File;
use std::io::{self, BufRead};

pub struct Input {
    pub data: Vec<String>,
}

impl Input {

    pub fn read(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        reader.lines().for_each(|line| {
            match line {
                Ok(content) => self.data.push(content),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        });

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            data: vec!()
        }
    }
}
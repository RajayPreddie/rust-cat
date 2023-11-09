use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

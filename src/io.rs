use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        lines.push(line.clone());
        line.clear();
    }
    Ok(lines)
}

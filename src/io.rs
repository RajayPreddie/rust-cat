use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
// TODO: Add a function that handles reading over multiple files in a directory


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_lines() {


        fn test_assert_lines(file_path: &str) {
            let lines = read_lines(file_path).unwrap();
            assert_eq!(lines.len(), 3);
            assert_eq!(lines[0], "Hello World");
            assert_eq!(lines[1], "Hi There");
            assert_eq!(lines[2], "Hello There");
        }
        test_assert_lines("test_data/test1.txt");
       
    }

}
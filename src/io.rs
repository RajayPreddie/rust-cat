//! # I/O Module for RustCat
//!
//! This module handles the input/output operations for the RustCat application.
//! It primarily deals with reading files and processing their contents into a usable format.
//! The functionality here is essential for the core operation of RustCat, facilitating
//! the reading of file lines to be processed and displayed.
use std::fs::File;
use std::io::{self, BufRead, BufReader};
/// Reads all lines from a specified file and returns them.
///
/// This function opens the file specified by `filename` and reads it line by line,
/// accumulating the lines into a `Vec<String>`. It uses buffered reading to efficiently
/// handle large files. In case of any I/O errors, the function returns an `io::Result`.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the path to the file to be read.
///
/// # Returns
///
/// Returns an `io::Result<Vec<String>>` which is a vector of lines in case of success,
/// or an I/O error in case of failure.
///
/// # Examples
///
/// ```
/// let lines = read_lines("example.txt").expect("Failed to read the file");
/// ```
pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {


    let file = File::open(filename)?; // Open the file
    let mut reader = BufReader::new(file); // Create a buffered reader
    let mut lines = Vec::new();// Create a vector to hold the lines
    let mut line = String::new(); // Create a string to hold each line

    // Read each line and push it to the vector
    while reader.read_line(&mut line)? > 0 {
        lines.push(line.clone());
        line.clear();
    }
    Ok(lines)
}

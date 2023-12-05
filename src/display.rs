//! # Display Module for RustCat
//!
//! This module is responsible for managing the display output of the RustCat application.
//! It handles the formatting and presentation of file contents based on the command-line
//! options specified by the user.
//!
//! The module leverages the `LineProcessor` for processing lines of the files and 
//! utilizes Rust's standard I/O capabilities for outputting the processed text.
use crate::args::Cli;
use crate::process_lines::LineProcessor;
use std::io::{self};
use std::process;
/// Displays the output for the given filenames according to the specified CLI options.
///
/// This function iterates over each filename, reads its content, processes it using
/// `LineProcessor`, and then displays the processed lines. It handles I/O errors and
/// exits the process if any occur while reading the files.
///
/// # Arguments
///
/// * `filenames` - A slice of `String` containing the paths of the files to be processed and displayed.
/// * `cli` - A reference to the `Cli` struct containing the command-line options.
///
/// # Errors
///
/// Exits the process with an error message if there's an issue reading any of the files.
pub fn display_output(filenames: &[String], cli: &Cli) {
    let stdout = io::stdout(); // Get handle to stdout
    let mut handle = stdout.lock(); // Lock the handle to stdout
    let mut line_processor = LineProcessor::new(cli); // Initialize the line processor
    // Iterate over each filename
    for filename in filenames {
        // Read the lines of the file
        match super::io::read_lines(filename) {
            Ok(lines) => {
                // Process and display the lines
                line_processor.process_and_display_lines(&lines, &mut handle);
            }
            // Handle I/O errors
            Err(e) => {
                eprintln!(
                    "rustcat: {}: {}",
                    filename,
                    e.to_string().split(" (os error").next().unwrap_or(&e.to_string())
                );
                process::exit(1);
            }
        }
    }
}

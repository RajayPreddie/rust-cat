//! # RustCat Main Module
//!
//! The main module of the RustCat application, an enhanced reimplementation of the Unix `cat` command.
//! This module acts as the entry point for the application, orchestrating the overall functionality.
//! It integrates various sub-modules like argument parsing (`args`), output display (`display`),
//! file input/output operations (`io`), and line processing (`process_lines`).
//!
//! The main functionality includes reading files, concatenating their contents, providing line numbering,
//! highlighting search terms, and syntax highlighting for various programming languages.
use args::Cli;
mod args;
mod display;
mod io;
mod process_lines;
/// The entry point of the RustCat application.
///
/// Initializes the application by parsing command-line arguments using the `Cli` struct from the `args` module.
/// It then passes the parsed arguments to the display module for processing and output display.
///
/// # Example
///
/// Running RustCat with a file argument:
/// ```bash
/// rustcat myfile.txt
/// ```
/// This will display the content of `myfile.txt` using RustCat's enhanced display features.
fn main() {
    let cli = Cli::new(); // Initializes command-line arguments
    display::display_output(&cli.files, &cli); // Calls the display module to process and output the file contents
}

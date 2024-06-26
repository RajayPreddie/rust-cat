//! # Line Processing Module for RustCat
//!
//! This module is dedicated to processing the lines of files based on the command-line options specified in RustCat.
//! It includes the `LineProcessor` struct, which encapsulates the logic for various text processing features like
//! line numbering, syntax highlighting, non-printing character display, and search term highlighting.
//!
//! The module leverages the `syntect` library for syntax highlighting and provides custom logic for other text
//! processing functionalities.

use crate::args::Cli;
use std::io::{self, Write};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

pub struct LineProcessor<'a> {
    /// Field definitions
    /// -----------------
    /// A reference to the `Cli` struct containing the command-line options.
    pub cli: &'a Cli,
    /// The current line number of the file being processed.
    pub file_line_number: usize,
    /// The number of consecutive blank lines encountered.
    pub number_of_consecutive_blank_lines: usize,
    /// The syntax set for syntax highlighting.
    pub syntax_set: SyntaxSet,
    /// The theme set for syntax highlighting.
    pub theme_set: ThemeSet,
}

impl<'a> LineProcessor<'a> {
    /// Constructs a new `LineProcessor`.
    ///
    /// Initializes syntax and theme sets for syntax highlighting and sets the initial state for line processing.
    pub fn new(cli: &'a Cli) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines(); // Initialize syntax set
        let theme_set = ThemeSet::load_defaults(); // Initialize theme set
        LineProcessor {
            cli,
            file_line_number: 1,
            number_of_consecutive_blank_lines: 0,
            syntax_set,
            theme_set,
        }
    }

    /// Checks if the given character is a non-printing character.
    fn is_nonprinting_char(&self, c: char) -> bool {
        c.is_control() && c != '\n' && c != '\t'
    }

    /// Checks if the given string contains any non-printing characters.
    fn contains_nonprinting_chars(&self, s: &str) -> bool {
        s.chars().any(|c| self.is_nonprinting_char(c))
    }
    /// Handles blank lines based on the specified command-line options.
    fn handle_blank_lines(&mut self, line: &str) {
        // Blank line does not contain any non-printing characters or tabs
        // Blank line needs to be empty or contain only whitespace
        if !self.contains_nonprinting_chars(line) && !line.contains('\t') && line.trim().is_empty()
        {
            self.number_of_consecutive_blank_lines += 1;
        } else {
            self.number_of_consecutive_blank_lines = 0;
        }
    }

    /// Increments the file line number.
    fn increment_line_number(&mut self) {
        self.file_line_number += 1;
    }
    /// Adds line numbers to the given line.
    fn add_line_numbers(&mut self, line: &str) -> String {
        // Format the line number and line
        let processed_line = format!("{:>6}\t{}", self.file_line_number, line).to_string();
        self.increment_line_number();
        processed_line
    }
    /// Shows the line numbers for the given line if the corresponding CLI option is enabled.
    fn show_line_numbers(&mut self, line: &str) -> String {
        let mut processed_line = line.to_string();
        if self.cli.show_line_numbers {
            processed_line = self.add_line_numbers(line)
        }
        processed_line
    }
    /// Shows the line numbers for non-blank lines only if the corresponding CLI option is enabled.
    fn show_non_blank_line_numbers(&mut self, line: &str) -> String {
        let mut processed_line = line.to_string();

        // Check if the show_non_blank_line_numbers option is enabled
        if self.cli.show_non_blank_line_numbers
        // Check if the line is not empty
            && (!line.trim().is_empty()
            // Check if the line contains non-printing characters or tabs
                || self.contains_nonprinting_chars(line)
                || line.contains('\t'))
        {
            processed_line = self.add_line_numbers(line);
        }
        processed_line
    }
    /// Shows the line end symbol `$` for the given line if the corresponding CLI option is enabled.
    fn show_ends(&self, line: &str) -> String {
        let mut processed_line = line.to_string();
        // Check if the show_ends option is enabled
        // Check if the line ends with a newline before adding the symbol.
        if self.cli.show_ends && line.ends_with('\n') {
            processed_line = line.replace('\n', "$\n");
        }

        processed_line
    }
    /// Displays non-printing characters for the given string.
    fn display_nonprinting_chars(&self, s: &str) -> String {
        let mut displayed = String::new();

        for c in s.chars() {
            // Check if the character is a non-printing character
            if c.is_control() && c != '\n' && c != '\t' {
                // Handle non-printing characters
                if c as u32 <= 31 {
                    displayed.push_str(&format!("^{}", (c as u8 + 64) as char));
                } else if c as u32 == 127 {
                    displayed.push_str("^?");
                } else {
                    // For extended characters
                    displayed.push_str(&format!("M-{}", c));
                }
            } else {
                // Handle printable characters
                displayed.push(c);
            }
        }

        displayed
    }
    /// Shows non-printing characters for the given line if the corresponding CLI option is enabled.
    fn show_nonprinting(&self, line: &str) -> String {
        let mut processed_line = line.to_string();
        if self.cli.show_nonprinting {
            processed_line = self.display_nonprinting_chars(line);
        }
        processed_line
    }
    /// Shows tabs as `^I` for the given line if the corresponding CLI option is enabled.
    fn show_tabs(&self, line: &str) -> String {
        let mut processed_line = line.to_string();
        if self.cli.show_tabs {
            processed_line = line.replace('\t', "^I");
        }
        processed_line
    }
    /// Highlights the syntax of the given line if the corresponding CLI option is enabled.
    fn highlight_line(&self, syntax_set: &SyntaxSet, theme: &Theme, line: &str) -> String {
        // Initialize the highlighter
        let mut h = match syntax_set.find_syntax_by_extension("rs") {
            Some(syntax) => HighlightLines::new(syntax, theme),
            None => {
                // Handle the error, e.g., use a default syntax
                let default_syntax = syntax_set.find_syntax_plain_text();
                HighlightLines::new(default_syntax, theme)
            }
        };
        // Highlight the line
        match h.highlight_line(line, syntax_set) {
            Ok(ranges) => syntect::util::as_24_bit_terminal_escaped(&ranges[..], false),
            Err(e) => {
                eprintln!("Error: {}", e);
                line.to_string()
            }
        }
    }
    /// Highlights the syntax of the given line if the corresponding CLI option is enabled.
    fn highlight_syntax(&self, line: &str) -> String {
        let mut processed_line = line.to_string();
        if self.cli.highlight_syntax {
            processed_line = self.highlight_line(
                &self.syntax_set,
                &self.theme_set.themes["base16-ocean.dark"],
                line,
            );
        }
        processed_line
    }
    /// Highlights the search term in the given line.
    fn highlight_search_term(line: &str, search_term: &str) -> String {
        let mut highlighted_line = String::new();
        let mut start = 0;

        while let Some(position) = line[start..].find(search_term) {
            // Add the text before the search term
            highlighted_line.push_str(&line[start..start + position]);

            // Add the search term with highlighting
            highlighted_line.push_str("\x1b[33m"); // Red color
            highlighted_line.push_str(search_term);
            highlighted_line.push_str("\x1b[0m"); // Reset color

            // Update the start position
            start += position + search_term.len();
        }

        // Add any remaining text after the last occurrence
        highlighted_line.push_str(&line[start..]);

        highlighted_line
    }
    /// Checks if the number of consecutive blanks lines has exceeded 1. If so, then it decrements the counter and returns `true` to skip the current line.
    pub fn is_skipping_blank_line(&mut self) -> bool {
        if self.cli.squeeze_blank && self.number_of_consecutive_blank_lines > 1 {
            self.number_of_consecutive_blank_lines -= 1;
            true
        } else {
            false
        }
    }
    /// Processes the given line based on the specified command-line options.
    fn process_line(&mut self, line: &str) -> String {
        let mut processed_line = line.to_string();

        processed_line = self.show_non_blank_line_numbers(&processed_line);
        processed_line = self.show_nonprinting(&processed_line);
        processed_line = self.show_line_numbers(&processed_line);
        processed_line = self.show_ends(&processed_line);
        processed_line = self.show_tabs(&processed_line);
        processed_line = self.highlight_syntax(&processed_line);
        processed_line
    }

    /// Processes,searches, and displays the given lines based on the specified command-line options.
    fn process_and_display_lines_search(
        &mut self,
        lines: &[String],
        handle: &mut io::StdoutLock<'_>,
        term: &str,
    ) {
        for line in lines {
            // Handle blank lines
            self.handle_blank_lines(line);

            // Skip the line if necessary
            if self.is_skipping_blank_line() {
                continue;
            }
            // Check if the line contains the search term
            if line.contains(term) {
                let mut processed_line = self.process_line(line);
                processed_line = Self::highlight_search_term(&processed_line, term);
                if let Err(e) = write!(handle, "{}", processed_line) {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
            } else {
                self.increment_line_number();
                continue;
            }
        }
    }
    /// Processes and displays the given lines based on the specified command-line options.
    fn process_and_display_lines_no_search(
        &mut self,
        lines: &[String],
        handle: &mut io::StdoutLock<'_>,
    ) {
        for line in lines {
            // Handle blank lines
            self.handle_blank_lines(line);
            // Skip the line if necessary
            if self.is_skipping_blank_line() {
                continue;
            }
            // Process and display the line
            let processed_line = self.process_line(line);
            if let Err(e) = write!(handle, "{}", processed_line) {
                eprintln!("Error writing to stdout: {}", e);
                break;
            }
        }
    }
    /// Processes and displays the given lines based on the specified command-line options. Decides whether to search or not.
    pub fn process_and_display_lines(&mut self, lines: &[String], handle: &mut io::StdoutLock<'_>) {
        // Check if the search option is enabled
        if let Some(term) = &self.cli.search {
            self.process_and_display_lines_search(lines, handle, term);
        } else {
            self.process_and_display_lines_no_search(lines, handle);
        }
    }
}

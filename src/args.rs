//! # Argument Parsing Module for RustCat
//!
//! This module utilizes `clap` for parsing command-line arguments for the RustCat application.
//! It defines the `Cli` struct which represents the possible command-line options that RustCat accepts.
//! This includes options for file display, line numbering, search functionality, and various formatting preferences.
//!
//! The `Cli` struct is also responsible for handling combined or conflicting command-line options and setting
//! appropriate flags for other parts of the application to use.

use clap::Parser;
/// Command-line arguments structure for RustCat.
///
/// This structure defines all the possible command-line options that can be passed to RustCat.
/// It uses `clap` for parsing and handling these options.
///
/// # Fields
///
/// * `files` - Files to display.
/// * `search` - Optional search term for highlighting within the file content.
/// * `show_line_numbers` - Flag to display line numbers.
/// * `show_non_blank_line_numbers` - Flag to display line numbers for non-blank lines only.
/// * `show_ends` - Flag to show `$` at the end of each line.
/// * `show_nonprinting` - Flag to display non-printing characters.
/// * `squeeze_blank` - Flag to suppress consecutive blank lines.
/// * `show_tabs` - Flag to display tabs as `^I`.
/// * `show_nonprinting_and_ends` - Combined flag for non-printing characters and line end symbol.
/// * `show_nonprinting_and_tabs` - Combined flag for non-printing characters and tab symbol.
/// * `show_all` - Flag to enable all display options.
/// * `highlight_syntax` - Flag to enable syntax highlighting.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Field Definitions
    ///
    /// Files to display
    #[arg(required = true)]
    pub files: Vec<String>,
    /// Optional search term for highlighting within the file content.
    #[arg(short = 'f', long = "search")]
    pub search: Option<String>,
    /// Flag to display line numbers.
    #[arg(short = 'n', long = "number")]
    pub show_line_numbers: bool,
    /// Flag to display line numbers for non-blank lines only.
    #[arg(short = 'b', long = "number-nonblank")]
    pub show_non_blank_line_numbers: bool,
    /// Flag to show `$` at the end of each line.
    #[arg(short = 'E', long = "show-ends")]
    pub show_ends: bool,
    /// Flag to display non-printing characters.
    #[arg(short = 'v', long = "show-nonprinting")]
    pub show_nonprinting: bool,
    /// Flag to suppress consecutive blank lines.
    #[arg(short = 's', long = "squeeze-blank")]
    pub squeeze_blank: bool,
    /// Flag to display tabs as `^I`.
    #[arg(short = 'T', long = "show-tabs")]
    pub show_tabs: bool,
    /// Combined flag for non-printing characters and line end symbol.
    #[arg(short = 'e', long = "show-nonprinting-and-ends")]
    pub show_nonprinting_and_ends: bool,
    /// Combined flag for non-printing characters and tab symbol.
    #[arg(short = 't', long = "show-nonprinting-and-tabs")]
    pub show_nonprinting_and_tabs: bool,
    /// Flag to enable all display options.
    #[arg(short = 'A', long = "show-all")]
    pub show_all: bool,
    /// Flag to enable syntax highlighting.
    #[arg(short = 'x', long = "highlight-syntax")]
    pub highlight_syntax: bool,
}

impl Cli {
    /// Constructs a new instance of `Cli`, parsing the command-line arguments.
    ///
    /// This method also handles the adjustment of combined flags and overrides
    /// for specific options to ensure correct functionality based on the provided arguments.
    pub fn new() -> Self {
        // Parse command-line arguments
        let mut cli = Cli::parse();
        // Adjust combined flags
        cli.adjust_combined_flags();
        // Override show_line_numbers
        cli.override_show_line_numbers();

        cli
    }
    /// Adjusts flags for combined command-line options.
    ///
    /// This method sets the necessary individual flags when combined flags are used.
    /// For example, enabling `show_nonprinting_and_ends` will set both `show_nonprinting`
    /// and `show_ends` to `true`.
    fn adjust_combined_flags(&mut self) {
        // Adjust combined flags
        if self.show_nonprinting_and_ends {
            self.show_nonprinting = true;
            self.show_ends = true;
        }

        if self.show_nonprinting_and_tabs {
            self.show_nonprinting = true;
            self.show_tabs = true;
        }
        if self.show_all {
            self.show_nonprinting = true;
            self.show_ends = true;
            self.show_tabs = true;
        }
    }
    /// Overrides the `show_line_numbers` flag if `show_non_blank_line_numbers` is set.
    ///
    /// This ensures that non-blank line numbers are shown correctly without conflicting
    /// with the general line number display.
    fn override_show_line_numbers(&mut self) {
        if self.show_non_blank_line_numbers {
            self.show_line_numbers = false;
        }
    }
}

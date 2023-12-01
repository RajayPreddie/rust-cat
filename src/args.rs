use clap::Parser;
use std::path::{Path, PathBuf};

/// `rustcat` is a command-line utility for displaying file contents and more.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Files to display
    #[arg(required = true)]
    pub files: Vec<String>,
    /// Search term
    #[arg(short ='f', long ="search")]
    pub search: Option<String>,

    #[arg(short='n', long="number")]
    pub show_line_numbers: bool,

    #[arg(short='b', long="number-nonblank")]
    pub show_non_blank_line_numbers: bool,


    #[arg(short = 'E', long = "show-ends")]
    pub show_ends: bool,
    #[arg(short = 'v', long = "show-nonprinting")]
    pub show_nonprinting: bool,

    #[arg(short = 's', long = "squeeze-blank")]
    pub squeeze_blank: bool,

    #[arg(short = 'T', long = "show-tabs")]
    pub show_tabs: bool,


    #[arg(short='e', long="show-nonprinting-and-ends")]
    pub show_nonprinting_and_ends: bool,


    #[arg(short='t', long="show-nonprinting-and-tabs")]
    pub show_nonprinting_and_tabs: bool,


    #[arg(short='A', long="show-all")]
    pub show_all: bool,


    #[arg(short='V', long="version")]
    pub version: bool,

}

impl Cli {
    pub fn new() -> Self {
        let mut cli = Cli::parse();
        cli.adjust_combined_flags();
        cli.override_show_line_numbers();
        cli
    }
     fn adjust_combined_flags(&mut self) {
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

     fn override_show_line_numbers(&mut self) {
        if self.show_non_blank_line_numbers {
            self.show_line_numbers = false;
        }
    }
}

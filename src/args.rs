use clap::Parser;
use std::path::{Path, PathBuf};

/// `rustcat` is a command-line utility for displaying file contents and more.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Files to display
    #[arg(required = true)]
    pub files: Vec<String>,


    /// Search term
    #[arg(short, long)]
    pub search: Option<String>,

    // TODO: Add a `--number` flag to display line numbers
    #[arg(short, long)]
    pub number: bool,

    /*
    TODO:
    Displaying File Contents: The basic function of cat is to read and display the contents of files. If no file is specified, it reads from standard input.
Concatenating Files: cat can concatenate the contents of multiple files, displaying them in the order they were provided.
Numbering Lines: The -n or --number option numbers all output lines, while -b or --number-nonblank numbers only non-empty lines, overriding -n​​.
Showing Ends of Lines: The -E or --show-ends option displays a $ character at the end of each line, making it easier to visualize line breaks​​.
Showing Tabs: With -T or --show-tabs, cat displays TAB characters as ^I, which can be useful for identifying tabs in files​​.
Squeezing Blank Lines: The -s or --squeeze-blank option suppresses repeated adjacent empty lines​​.
Displaying Non-Printing Characters: The -v or --show-nonprinting option shows non-printing characters (except line feeds and tabs) using ^ and M- notation​​.
Showing All: The -A or --show-all option is equivalent to -vET, combining the effects of showing non-printing characters, ends of lines, and tabs​​.
Output Version Information: Using --version displays the version information of the cat command​​.
Help Information: The --help option displays usage information for the cat command​​.
     */
}

pub fn parse_args() -> Cli {

    Cli::parse()
}

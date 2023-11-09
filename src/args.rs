use clap::Parser;

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
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

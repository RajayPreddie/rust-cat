use args::Cli;

mod args;
mod display;
mod io;
mod process_lines;

fn main() {
    let cli = Cli::new();
    display::display_output(&cli.files, &cli);
}

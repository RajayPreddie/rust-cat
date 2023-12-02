use args::Cli;

mod args;
mod display;
mod io;
mod process_lines;
// TODO: Modularize further if possible
// TODO: Add Documentation
// TODO: Testing: Run cat command on test files and compare output
fn main() {

    let cli = Cli::new();
    display::display_output(&cli.files, &cli);
    
}

mod args;
mod display;
mod io;
mod search;


fn main() {

    let cli = args::parse_args();
    // TODO: Prompt ChatGPT about how to handle different types of argument parsing/errors
    // TODO: Add a function that handles the different cli arguments
    // TODO: show line numbers for normal output and search
    // TODO: add a --syntax flag for syntax highlighting
    if let Some(term) = &cli.search {
        search::search_and_display(&cli.files, term);
    } else {
        // TODO: iterate through each file and then each files lines
        display::display_files(&cli.files);
    }
}

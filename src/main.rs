mod args;
mod display;
mod io;
mod search;

fn main() {
    let cli = args::parse_args();

    if let Some(term) = &cli.search {
        search::search_and_display(&cli.files, term);
    } else {
        display::display_files(&cli.files);
    }
}

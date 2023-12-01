use args::Cli;

mod args;
mod display;
mod io;
mod search;

fn main() {

    let cli = Cli::new();
    
    if let Some(term) = &cli.search {
        search::search_and_display(&cli.files, term);
    } else {
        display::parse_args(&cli.files, &cli);
    }
}

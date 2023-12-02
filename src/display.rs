use std::io::{self, Write};
use std::process;
use crate::args::Cli;
use crate::process_lines::LineProcessor;


pub fn display_output(filenames: &[String], cli: &Cli) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut line_processor = LineProcessor::new(cli);

    for filename in filenames {
        match super::io::read_lines(filename) {
            Ok(lines) => {

                 line_processor.process_and_display_lines(&lines, &mut handle);

            } 
            Err(e) => {eprintln!("rustcat: {}: {}", filename,e.to_string().split(" (os error").next().unwrap_or(""));
            process::exit(1);
        },
        }
    } 
}
// Path: src/io.rs
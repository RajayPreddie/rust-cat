use std::io::{self, Write};
use crate::args::Cli;
use crate::process_lines::LineProcessor;




pub fn parse_args(filenames: &[String], cli: &Cli) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut line_processor = LineProcessor::new(cli);

    for filename in filenames {
        match super::io::read_lines(filename) {
            Ok(lines) => {
                if let Err(e) = writeln!(handle, "==> {} <==", filename) {
                    eprintln!("Error writing to stdout: {}", e);
                    continue;
                }

                 line_processor.process_and_display_lines(&lines, &mut handle);

            } 
            Err(e) => eprintln!("rustcat: {}: {}", filename, e),
        }
    }
    
}
// Path: src/io.rs
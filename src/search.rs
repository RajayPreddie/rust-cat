use std::io::{self, Write};

pub fn search_and_display(filenames: &[String], term: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for filename in filenames {
        match super::io::read_lines(filename) {
            Ok(lines) => {
                let matches: Vec<_> = lines
                    .into_iter()
                    .filter(|line| line.contains(term))
                    .collect();

                if !matches.is_empty() {
                    writeln!(handle, "==> {} <==", filename).unwrap();
                    for line in matches {
                        writeln!(handle, "{}", line).unwrap();
                    }
                }
            }
            Err(e) => eprintln!("rustcat: {}: {}", filename, e),
        }
    }
}
// Path: src/search.rs
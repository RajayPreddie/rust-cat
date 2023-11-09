use std::io::{self, Write};

pub fn display_files(filenames: &[String]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for filename in filenames {
        match super::io::read_lines(filename) {
            Ok(lines) => {
                writeln!(handle, "==> {} <==", filename).unwrap();
                for line in lines {
                    writeln!(handle, "{}", line).unwrap();
                }
            }
            Err(e) => eprintln!("rustcat: {}: {}", filename, e),
        }
    }
}
// Path: src/io.rs
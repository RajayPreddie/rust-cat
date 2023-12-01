use std::io::{self, Write};
use crate::args::Cli;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;

fn highlight_line(syntax_set: &SyntaxSet, theme: &Theme, line: &str) -> String {
    let mut h = HighlightLines::new(syntax_set.find_syntax_by_extension("rs").unwrap(), theme);
    match h.highlight_line(line, syntax_set) {

        Ok(ranges) => syntect::util::as_24_bit_terminal_escaped(&ranges[..], false),
        Err(e) => {
            eprintln!("Error: {}", e);
            line.to_string()
        }
    }
}

fn display_nonprinting_chars(s: &str) -> String {
    let mut displayed = String::new();

    for c in s.chars() {
        if c.is_control() && c != '\n' && c != '\t' {
            // Handle non-printing characters
            if c as u32 <= 31 {
                displayed.push_str(&format!("^{}", (c as u8 + 64) as char));
            } else if c as u32 == 127 {
                displayed.push_str("^?");
            } else {
                // For extended characters
                displayed.push_str(&format!("M-{}", c));
            }
        } else {
            // Handle printable characters
            displayed.push(c);
        }
    }

    displayed
}


pub fn parse_args(filenames: &[String], cli: &Cli) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut file_line_number = 1;
    let mut number_of_consecutive_blank_lines = 0;
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["base16-ocean.dark"];
    
    for filename in filenames {
        match super::io::read_lines(filename) {
            Ok(lines) => {
                if let Err(e) = writeln!(handle, "==> {} <==", filename) {
                    eprintln!("Error writing to stdout: {}", e);
                    continue;
                }
                for mut line in lines {
                   
                   if line.is_empty() {
                    number_of_consecutive_blank_lines += 1;
                   }
                   else {
                     number_of_consecutive_blank_lines = 0;
                   }
                    if cli.show_non_blank_line_numbers  && !line.is_empty() {
                        line = format!("{:>4} {}", file_line_number, line);
                        file_line_number += 1;
                    }
                    if cli.squeeze_blank && number_of_consecutive_blank_lines > 1 {
                        number_of_consecutive_blank_lines -= 1;
                        continue;
                    }
                    if cli.show_line_numbers {
                        line = format!("{:>4} {}", file_line_number, line);
                        file_line_number += 1;
                    }
                    if cli.show_ends {
                        line = format!("{}$", line);
                    }
                    if cli.show_nonprinting {
                      
                        line = display_nonprinting_chars(&line);
                    }

                    if cli.show_tabs {
                        line = line.replace("\t", "^T");
                    }
                    if cli.highlight_syntax {
                        line = highlight_line(&syntax_set, theme, &line);
                    }
                
                    if let Err(e) = writeln!(handle, "{}", line) {
                        eprintln!("Error writing to stdout: {}", e);
                        break;
                    }
                }
            }
            Err(e) => eprintln!("rustcat: {}: {}", filename, e),
        }
    }
    
}
// Path: src/io.rs
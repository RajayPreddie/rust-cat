use std::io::{self, Write};
use crate::args::Cli;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;

pub struct LineProcessor<'a> {
  pub cli: &'a Cli,
  pub file_line_number: usize,
  pub number_of_consecutive_blank_lines: usize,
  pub syntax_set: SyntaxSet,
  pub theme_set: ThemeSet,
}

impl<'a> LineProcessor<'a> {
  pub fn new(cli: &'a Cli) -> Self {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    LineProcessor {
      cli,
      file_line_number: 1,
      number_of_consecutive_blank_lines: 0,
      syntax_set,
      theme_set,
    }
  }

  
  fn handle_blank_lines(&mut self, line: &str)  {
    if line.trim().is_empty() {
      self.number_of_consecutive_blank_lines += 1;
    }
    else {
      self.number_of_consecutive_blank_lines = 0;
    }
  }
  fn increment_line_number(&mut self) {
    self.file_line_number += 1;
  }
  fn add_line_numbers(&mut self, line: &str) -> String  {
      let processed_line = format!("{:>6}\t{}", self.file_line_number, line).to_string();
      self.file_line_number += 1;
      processed_line
  }
 
  fn show_line_numbers(&mut self,  line: &str) -> String  {
    let mut processed_line = line.to_string();
    if self.cli.show_line_numbers {
      processed_line = self.add_line_numbers(line)
    }
    processed_line
  }
  
  fn show_non_blank_line_numbers(&mut self, line: &str) -> String  {
    let mut processed_line = line.to_string();
    if self.cli.show_non_blank_line_numbers  && !line.trim().is_empty() {
      processed_line = self.add_line_numbers(line);
    }
    processed_line
  }
  
  fn show_ends(&self, line: &str) -> String {
    let mut processed_line = line.to_string();
    if self.cli.show_ends && line.ends_with("\n") {
      processed_line = line.replace("\n", "$\n");
    }
  
    processed_line
  }
  
  fn display_nonprinting_chars(&self, s: &str) -> String {
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


  fn show_nonprinting(&self, line: &str) -> String  {
    let mut processed_line = line.to_string();
    if self.cli.show_nonprinting {
      processed_line = self.display_nonprinting_chars(line); 
    }
    processed_line
  }
  
  fn show_tabs(&self, line: &str) -> String  {
    let mut processed_line = line.to_string();
    if self.cli.show_tabs {
      processed_line = line.replace("\t", "^I");
    }
    processed_line
  }
 
  
  fn highlight_line(&self, syntax_set: &SyntaxSet, theme: &Theme, line: &str) -> String {
      let mut h = HighlightLines::new(syntax_set.find_syntax_by_extension("rs").unwrap(), theme);
      match h.highlight_line(line, syntax_set) {
  
          Ok(ranges) => syntect::util::as_24_bit_terminal_escaped(&ranges[..], false),
          Err(e) => {
              eprintln!("Error: {}", e);
              line.to_string()
          }
      }
  }
  fn highlight_syntax(&self, line: &str) -> String  {
    let mut processed_line = line.to_string();
    if self.cli.highlight_syntax {
      processed_line= self.highlight_line(&self.syntax_set, &self.theme_set.themes["base16-ocean.dark"], &line);
    }
    processed_line
  }

  fn highlight_search_term(line: &str, search_term: &str) -> String {
    let mut highlighted_line = String::new();
    let mut start = 0;

    while let Some(position) = line[start..].find(search_term) {
        // Add the text before the search term
        highlighted_line.push_str(&line[start..start + position]);

        // Add the search term with highlighting
        highlighted_line.push_str("\x1b[33m"); // Red color
        highlighted_line.push_str(search_term);
        highlighted_line.push_str("\x1b[0m"); // Reset color

        // Update the start position
        start += position + search_term.len();
    }

    // Add any remaining text after the last occurrence
    highlighted_line.push_str(&line[start..]);

    highlighted_line
}


  pub fn is_skipping_blank_line(&mut self) -> bool {
    if self.cli.squeeze_blank && self.number_of_consecutive_blank_lines > 1 {
      self.number_of_consecutive_blank_lines -= 1;
      true
    }
    else {
    false
    }
  }
  
   fn process_line(&mut self, line: &str) -> String {
    
    let mut processed_line = line.to_string();
    
    processed_line = self.show_non_blank_line_numbers(&processed_line);

    processed_line = self.show_nonprinting(&processed_line);
    processed_line = self.show_line_numbers(&processed_line);
    processed_line = self.show_ends(&processed_line);
    processed_line = self.show_tabs(&processed_line);

    processed_line = self.highlight_syntax(&processed_line);
    
    processed_line
  }
   fn process_and_display_lines_search(&mut self, lines: &[String], handle: &mut io::StdoutLock<'_>, term: &str) {
    for line in lines {
      self.handle_blank_lines(line);
      if self.is_skipping_blank_line() {
        continue;
      }
   
        if line.contains(term) {
          let mut processed_line = self.process_line(&line);
          processed_line = Self::highlight_search_term(&processed_line, term);
          if let Err(e) = writeln!(handle, "{}", processed_line) {
            eprintln!("Error writing to stdout: {}", e);
            break;
          }
        
        }
        else {
          self.increment_line_number();
          continue;
          }
        
    } 
    
    }

  fn process_and_display_lines_no_search(&mut self, lines: &[String], handle: &mut io::StdoutLock<'_>) {
    for line in lines {
      self.handle_blank_lines(line);
      if self.is_skipping_blank_line() {
        continue;
      }
    

       let processed_line = self.process_line(&line);

       
     

      if let Err(e) = write!(handle, "{}", processed_line) {
        eprintln!("Error writing to stdout: {}", e);
        break;
      }
    }
    
  }

  pub fn process_and_display_lines(&mut self, lines: &[String], handle: &mut io::StdoutLock<'_>) {

      if let Some(term) = &self.cli.search{

        self.process_and_display_lines_search(lines, handle, term);
      }
      else {
        self.process_and_display_lines_no_search(lines, handle);
      }
  
  }

}
// Path: src/process_lines.rs



[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# RustCat

RustCat is an enhanced reimplementation of the Unix `cat` command, offering advanced features for modern use cases. It is designed to provide powerful capabilities for file concatenation, line numbering, file search, and more, while maintaining a user-friendly command-line interface. Currently, RustCat works with only files and does not work with standard input. RustCat reimplements most of the original cat features and adds additional features.

## Extended Features
- **Search Functionality:** Allows users to search and highlight specific terms within a file.
- **Syntax Highlighting:** Utilizes the `syntect` library for accurate syntax highlighting in various programming languages.

## Installation

To install RustCat, ensure Rust is installed on your system. Then use cargo to install:

```
cargo install rustcat
```

## How to Use

RustCat includes numerous command-line options for flexible file processing:

- **Display File:** `rustcat file` - Standard file display.
- **Concatenate Files:** `rustcat file1 file2...` - Concatenate and display multiple files.
- **Number Non-Blank Lines:** `rustcat -b file` or `rustcat --number-nonblank file` - Number non-blank lines only. This overrides -n
- **Display Non-Printing Characters:** 
  - `rustcat -e file` or `rustcat --show-nonprinting-and-ends file` - Show non-printing characters and `$` at line end. It is the same as -vE.
  - `rustcat -t file` or `rustcat --show-nonprinting-and-tabs file` - Display non-printing characters with tabs as `^I`. This is the same as -vT.
  - `rustcat -v file` or `rustcat --show-nonprinting file` - Display control characters and non-ASCII characters in a visible format.
- **Show Ends/Tabs:**
  - `rustcat -E file` or `rustcat --show-ends file` - Display `$` at the end of each line.
  - `rustcat -T file` or `rustcat --show-tabs file` - Show TAB characters as `^I`.
- **Number All Lines:** `rustcat -n file`or `rustcat --number file` - Number all output lines.
- **Squeeze Blank Lines:** `rustcat -s file` or `rustcat --squeeze-blank file` - Suppress multiple consecutive empty lines.
- **Show All:**  `ruscat -A file` or `rustcat --show-all file` is the same as -vET.
- **Syntax Highlighting:** `ruscat -x file` or `rustcat --highlight filename.rs` - Syntax highlighting for supported languages.
- **Search in File:** `rustcat -f "search term" file` or `rustcat --search "search term" file` - Highlight a specified search term within a file.
- **Help and Version Info:** 
  - `rustcat --help` - Display detailed usage instructions.
  - `rustcat --version` - Show the current version of RustCat.

## Testing

RustCat undergoes thorough integration testing, comparing its output with the Unix `cat` command to ensure feature compatibility and correctness.

## Next Steps
The next steps for this project would be the following:
1) Display input from standard input in addition to files.
2) Additional integration tests (search, multiple files, displaying standard input, syntax highlighting, highlighting search word)

# Acknowledgements
This project was developed with the aid of ChatGPT. Here are some of the ChatGPT links:
1) 
2) 
3) 
I also utilized the manual for Unix and Linux:
1) https://www.man7.org/linux/man-pages/man1/cat.1.html
2) https://ss64.com/bash/cat.html
3) https://www.unix.com/man-page/osx/1/cat/
4) man cat in Unix and Linux.

## License

RustCat is available under the [MIT License](LICENSE).

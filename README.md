[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# RustCat

## Description

`RustCat` is an advanced reimplementation of the Unix `cat` command with additional features, written in Rust. It's tailored for developers and terminal users who need more than just concatenating and displaying files. `RustCat` offers enhanced functionality such as line numbering, file concatenation with headers/seperators between each file content, and a basic search within files, while maintaining the simplicity and ease of use of the classic `cat` command. The idea for this project was developed with the aid of ChatGPT. Here is the link to the ChatGPT [conversation](https://chat.openai.com/share/79087c07-9344-4bce-b1f2-6763f1cc5132).


## Features

- **Basic File Display**: Like the traditional `cat`, `RustCat` will display the contents of files to the terminal window, maintaining the original layout and format.

- **Multiple File Handling**: Users can input several files at once, and `RustCat` will concatenate their contents in the given order to the standard output.

- **Line Numbering**: For ease of reference and debugging, `RustCat` can optionally prefix each line of output with its corresponding line number.

- **Concatenation with Headers**: When concatenating multiple files, `RustCat` can insert custom headers between files to clarify the transition from one file to the next, which is useful for reading combined outputs or logs.

- **Search Functionality**: `RustCat` includes a search feature that allows users to find and highlight specific terms within the file, which is a significant enhancement over the traditional `cat`.

## Installation

Ensure Rust is installed on your system before installing `RustCat`. Visit [The Rust Programming Language](https://www.rust-lang.org/tools/install) for installation instructions.

Install `RustCat` using cargo:

```sh
cargo install rustcat
```

## How to Use

Display the contents of a file:

```sh
rustcat file.txt
```

Concatenate and display multiple files with optional headers:

```sh
rustcat header1.txt file1.txt header2.txt file2.txt
```

Display file contents with line numbers:

```sh
rustcat -n file.txt
```

Search for and highlight a term within a file:

```sh
rustcat -s "search_term" file.txt
```

For detailed usage instructions, refer to the help command:

```sh
rustcat --help
```

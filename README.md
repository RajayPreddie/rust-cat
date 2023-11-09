[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# RustVMStat

## Description

`RustVMStat` is a command-line utility inspired by the classic `vmstat` tool, reimagined in Rust for modern Unix-like operating systems. It reports real-time statistics about system processes, memory, paging, block IO, traps, and CPU activity, catering to both system administrators and developers. This Rust implementation emphasizes an enhanced user experience with real-time updates and visualizations, harnessing the power of the `tui-rs` crate to render a terminal-based graphical user interface. The goal of `RustVMStat` is to provide a more interactive and visually appealing way to monitor system performance, with easy-to-read charts and customizable views. The idea for this project was developed with the aid of ChatGPT. Here is the link to the ChatGPT [conversation](https://chat.openai.com/share/2873ea3b-068b-49d5-849f-cba1257fcc4e).

## Installation

To compile and install `RustVMStat`, you will need Rust's package manager, `cargo`. You can install `cargo` and the Rust compiler (`rustc`) by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can compile `RustVMStat` with the following command:

```sh
cargo install rustvmstat
```

This command will download and compile `RustVMStat` and its dependencies, including `tui-rs`. After compilation, the `rustvmstat` binary will be available in your Rust binary folder, typically located at `$HOME/.cargo/bin`.

## How to Use

After installation, you can start `RustVMStat` by simply running the following command in your terminal:

```sh
rustvmstat
```

Upon launch, `RustVMStat` will display a dashboard with various system statistics. You can navigate through different views using your keyboard and configure the refresh rate or the metrics displayed according to your preferences.

Example use-cases:

- To monitor CPU and memory usage, `RustVMStat` will provide real-time graphs and raw data views.
- For disk IO monitoring, the tool will display read/write throughput and operations per second.
- System administrators can use `RustVMStat` to quickly check the overall health of the system or to diagnose performance bottlenecks.

Remember to check the `--help` option for more detailed command usage:

```sh
rustvmstat --help
```



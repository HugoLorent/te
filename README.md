# TE (Text Editor)

A lightweight terminal-based text editor written in Rust. This project is being developed by following the excellent [Build a Text Editor in Rust](https://flenker.blog/hecto/) tutorial by Philipp Flenker. My main goals are learning Rust, understanding how text editors work under the hood, and having fun while coding!

## Learning Project

This is primarily a learning project where I'm:
- Learning Rust programming concepts
- Understanding terminal manipulation
- Discovering how text editors work internally
- Following best practices and writing clean code

I'm building this editor step by step following the Hecto tutorial, which is an amazing resource for anyone interested in both Rust and text editor development.

## Features

- Terminal-based user interface
- Raw mode terminal handling
- Basic cursor movements
- Clean exit with CTRL+Q
- Cross-platform support

## Requirements

- Rust 1.56 or higher
- Terminal with ANSI support

## Installation

Clone the repository:
```bash
git clone https://github.com/yourusername/te.git
cd te
```

Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/te`

## Usage

Run the editor:
```bash
cargo run
```

Or if you've built it:
```bash
./target/release/te
```

### Key Bindings

- `Ctrl+Q`: Quit the editor
- Arrow keys: Move cursor (coming soon)
- Page Up/Down: Scroll through the document (coming soon)
- Home/End: Move to start/end of line (coming soon)

## Development

This project uses the following dependencies:
- `crossterm`: For terminal manipulation and event handling

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Based on the [Hecto tutorial](https://flenker.blog/hecto/) by Philipp Flenker
- Inspired by various terminal-based editors
- Built with Rust's powerful ecosystem

# Pyline — CLI Keyword Counter for Source Code

Pyline is a command-line tool for analyzing and counting keywords in source
code.
The project consists of a console binary `pyline-cli` and a library crate
`pyline-libs`.

## Features

- 📁 Recursive directory scanning with configurable exclusions
- 🔍 Smart file filtering by extension, directory, and filename patterns
- 📊 Detailed code statistics collection and analysis
- 🚀 Async-powered performance for large codebases
- ⚙️ Flexible CLI configuration with multiple filtering options
- 📈 Keyword frequency analysis across multiple programming languages
- 🎯 Support for multiple file formats and programming languages

## Quick Start

### Prerequisites

- Rust Toolchain (Rust and Cargo, version 1.75 or higher)
- [Installation instructions](https://rust-lang.org/tools/install/)

### Building from Source

The project uses Cargo workspaces to manage both the CLI binary and library
crate:

```bash
git clone https://github.com/Shindler7/pyline
cd pyline
cargo build --release
```

The compiled binaries will be located in target/release/.

### First Run

After building, you can run the application with the --help flag to
see available options:

```shell
# From the project root
./target/release/pyline --help
```

For development, you can run the CLI directly from its subdirectory:

```shell
cd pyline-cli
cargo run -- --help
```

## Development

The project is organized as a Cargo workspace with two main components:

* `pyline-cli/` — Command-line interface application
* `pyline-libs/` — Core library with analysis functionality

## Building for Development

```shell
# Build everything
cargo build

# Build and run tests
cargo test

# Run with specific input
cargo run -- --lang py --path ./my_project
```

## Usage Examples

```shell
# Analyze current directory
pyline --lang py

# Analyze specific directory
pyline --lang py --path /path/to/project

# Exclude test directories
pyline --lang py --dirs tests --dirs __pycache__

# Add file to collection by extensions
pyline --extension txt
```

## Roadmap

* Add support for more programming languages
* Implement JSON/CSV output formats
* Add progress indicators for large codebases
* Create configuration file support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

# Acknowledgments

Built with the amazing Rust programming language.

Inspired by various code analysis tools in the ecosystem.

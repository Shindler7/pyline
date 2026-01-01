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

### Supported Languages

Rust, Python

## Quick Start

### Prerequisites

- Rust Toolchain (Rust and Cargo, version 1.83 or higher)
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

### Example Usage

The simplest option: scan Python files with automatic configuration.

```shell
$ pyline --lang py --auto-config -p d:\coderep

Selected language: PYTHON, https://www.python.org/

The files in the directory are being examined: d:\coderep

Gathering files for analysis... OK. Successfully gathered 450 files.

Gathering code stats... OK.
Files: 450
Lines: 40396
  of which are code lines: 38768

Keywords:
  def = 2014
  ...
```

**Note**: In the example above, the -p key was used with a folder reference.
If you need to analyze the current folder, you can omit the key:

```shell
# short
$ pyline -l rust -a

# long
$ pyline --lang rust --auto-config
```

Let's make it more complex. For example, we don't want to scan directories
where a main.py file is found. Then we do this:

```shell
$ pyline --lang py --auto-config -p d:\coderep -m main.py

Selected language: PYTHON, https://www.python.org/

The files in the directory are being examined: d:\coderep

Gathering files for analysis... OK. Successfully gathered 450 files.

Gathering code stats... OK.
Files: 450
Lines: 40396
  of which are code lines: 38768

Keywords:
  def = 2014
  ...
```

**Note**: In the second example, the number of files may be lower if
directories
containing main.py are excluded from scanning.

### Key Features

- **Language-aware analysis** with predefined language profiles (`--lang`)
- **Automatic configuration** based on language conventions (`--auto-config`)
- **Smart directory traversal** with multiple exclusion mechanisms:
    - Exclude specific directories (`--exclude-dirs`)
    - Skip directories containing marker files (`--marker-files`)
    - Automatic dot-directory filtering (`--ignore-dot-dirs`)
- **Flexible file filtering** by extensions (`--ext`) and filenames (
  `--exclude-files`)
- **Detailed statistics** including line counts, code lines, and keyword
  frequencies
- **Verbose mode** for debugging and detailed progress information (
  `--verbose`)

## Roadmap

* Add support for more programming languages
* Implement JSON/CSV output formats
* Add progress indicators for large codebases
* Create configuration file support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Versioning

This project uses independent versioning for each crate in the workspace:

* **`pyline-cli`** — `0.3.0`
* **`pyline-libs`** — `0.3.2`

### Changelog

#### libs-0.3.2

- A minor bug introduced during the refactoring for Linux compatibility
  has been fixed.

#### libs-0.3.1

- For Python parsing, clarified handling of intermediate symbols (such
  as commas, parentheses) that are ignored and reset the accumulated
  keyword buffer

#### 0.3.0

- Added support for parsing Rust files
- Unified parsing methods, including macro creation, for easier expansion
  of supported languages

#### 0.2.0

**pyline-cli**

- Added CLI flags: `--ignore-dot-dirs`, `--auto-config`, `--marker-files`
- Renamed several existing flags for clarity
- Added directory exclusion when marker files are detected
- Added automatic file collection configuration via `--auto-config`
- Added comprehensive test suite
- Internal refactoring and improvements

**pyline-libs**

- Updated to support new CLI features
- Minor internal adjustments

## License

This project is licensed under the MIT License.

## Acknowledgments

Built with the amazing Rust programming language.

Inspired by various code analysis tools in the ecosystem.

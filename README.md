# Pyline — CLI Keyword Counter for Source Code

Pyline is a command-line tool for analyzing and counting keywords in source
code. The project consists of a console binary `pyline-cli` and a library crate
`pyline-libs`.

## Features

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

### Supported Languages

* Rust
* Python

## Quick Start

### Prerequisites

- Rust toolchain (Rust and Cargo, version 1.83 or higher)

```shell
user@WSMegaLand:/$ cargo version
cargo 1.96.0 (30a34c682 2026-05-25)
user@WSMegaLand:/$
```

- [Installation instructions](https://rust-lang.org/tools/install/)

### Building from Source

The project uses Cargo workspaces to manage both the CLI binary and library
crate:

```bash
git clone https://github.com/Shindler7/pyline
cd pyline
cargo build --release
```

The compiled binaries will be located in `target/release/`.

### Installing

To install the `pyline` binary into your Cargo bin directory (usually
`~/.cargo/bin`), run from the project root:

```shell
cargo install --path ./pyline-cli
```

After that, `pyline` is available on your `PATH`:

```shell
pyline --help
```

### First Run

If you installed the binary, just run:

```shell
pyline --help
```

For development, you can run the CLI directly from its subdirectory:

```shell
cd pyline-cli
cargo run -- --help
```

### Example Usage

The simplest option: scan Python files with automatic configuration.

```shell
$ pyline -l py -a -p d:\coderep

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

**Note**: In the example above, `-p` was used with an explicit path. To analyze
the current directory, omit it:

```shell
# short
$ pyline -l rust -a

# long
$ pyline --lang rust --auto-config
```

Let's make it more complex. For example, we don't want to scan directories
where a main.py file is found. Then we do this:

```shell
$ pyline --lang py --auto-config -p d:\coderep --marker-files .noscan

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
directories containing main.py are excluded from scanning.

## Documentation

API documentation for both crates can be built locally:

```shell
cargo doc --no-deps
```

The generated HTML is placed in `target/doc/`. To open it in your browser:

```shell
cargo doc --no-deps --open
```

## Roadmap

* Add support for more programming languages
* Implement JSON/CSV output formats
* Create configuration file support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Versioning

Each crate in the workspace is versioned independently:

* **`pyline-cli`** — `0.1.0`
* **`pyline-libs`** — `0.1.0`

> **Note:** Versioning was reset to `0.1.0` after a major refactoring.
> The previous `0.4.x` / `0.3.x` history was discarded as premature —
> the project is now being versioned anew from a clean baseline.

### Changelog

#### 0.1.0

Initial release under the new versioning scheme, following a full refactoring
of both crates. The pre-reset history is summarized below for reference.

## License

This project is licensed under the MIT License.

## Acknowledgments

Built with the amazing Rust programming language.

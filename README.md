# Rcat

A small `cat` clone written in Rust.

Rcat was built as a small Rust learning project to practice file handling,
command-line arguments, error handling, and buffered I/O.

## Features

- Read and print the contents of a file
- Validate that the provided path exists
- Reject directories as input
- Buffered file reading using `BufReader`
- Simple command-line interface
- Clear error messages with appropriate process exit codes

## Usage

Build the project with Cargo:

```bash
cargo build --release
```

Run Rcat by providing a file path:

```bash
cargo run -- <file>
```

Example:

```bash
cargo run -- Cargo.toml
```

Or use the compiled binary:

```bash
./target/release/rcat Cargo.toml
```

## Example

```text
$ rcat hello.txt

Hello, world!
This file was read using Rcat.
```

When no file path is provided:

```text
$ rcat
Please Provide A File Path.
```

When the specified path does not exist:

```text
$ rcat missing.txt
missing.txt File Does Not Exist!
```

When a directory is provided:

```text
$ rcat src
src Is A Directory!
```

## How It Works

Rcat follows a simple flow:

```text
Command-line argument
        ↓
Validate file path
        ↓
Open file
        ↓
Create buffered reader
        ↓
Read file contents
        ↓
Print contents
```

The file is opened using Rust's `std::fs::File` and read through
`std::io::BufReader`.

## Limitations

The current implementation reads files as UTF-8 text.

Because Rcat uses `read_to_string()`, arbitrary binary files are not supported
by the current version.

Rcat currently accepts exactly one file path argument.

## Building

### Requirements

- Rust
- Cargo

### Build

Clone the repository and build it:

```bash
git clone https://github.com/VJ132/Rcat.git
cd Rcat
cargo build --release
```

## Project Structure

```text
Rcat/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── LICENSE
└── README.md
```

## Purpose

This project is primarily a learning exercise while exploring Rust.

The goal is to keep the implementation small and understandable rather than
recreate every feature of the Unix `cat` utility.

## License

Rcat is licensed under the MIT License.

See [LICENSE](LICENSE) for the full license text.

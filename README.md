<h1 align="center">port-claim</h1>
<div align="center">
  <strong>If a port is in use, port-claim stops the process using it.</strong>
</div>
<br>
<div align="center">
  <a href="https://npmjs.org/package/port-claim">
    <img src="https://img.shields.io/npm/v/port-claim.svg?style=flat-square" alt="Package version" />
  </a>
  <a href="https://npmjs.org/package/port-claim">
    <img src="https://img.shields.io/npm/dm/port-claim.svg?style=flat-square" alt="Downloads" />
  </a>
  <a href="https://github.com/feross/standard">
    <img src="https://img.shields.io/badge/code%20style-standard-brightgreen.svg?style=flat-square" alt="Standard" />
  </a>
  <a href="https://github.com/jasenmichael/port-claim/blob/main/LICENSE">
    <img src="https://img.shields.io/npm/l/port-claim.svg?style=flat-square" alt="License" />
  </a>
  <a href="http://makeapullrequest.com">
    <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" alt="PRs" />
  </a>
</div>
<br>

## Installation

```bash
cargo install --path .
```

## Usage

```
port-claim -h | --help | -V | --version
port-claim <port> [<additional-ports>] [-v | --verbose]
```

### Options

- `-h, --help`: Prints usage information
- `-V, --version`: Prints the version
- `-v, --verbose`: Prints verbose information about port status and process killing

### Arguments

- `<port>`: Required argument specifying the port to check and kill if in use
- `[<additional-ports>]`: Optional additional ports to check and kill

## Examples

```bash
# Check if port 8080 is in use and kill the process if it is
port-claim 8080

# Check multiple ports with verbose output
port-claim 3000 8080 9000 --verbose

# Display help information
port-claim --help

# Display version information
port-claim --version
```

## How It Works

1. When you specify one or more ports, the tool checks if each port is available.
2. If a port is in use, the tool attempts to kill the process using that port.
3. With the `--verbose` flag, the tool will print information about each step of the process.

## Platform Support

The tool works on both Unix-based systems (Linux, macOS) and Windows, using different system commands to identify and kill processes on each platform. 

## Development

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/port-claim.git
cd port-claim

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
rustup component add clippy rustfmt
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test --verbose -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Check code formatting (without making changes)
cargo fmt -- --check

# Run clippy linter
cargo clippy

# Run clippy with strict warnings
cargo clippy -- -D warnings
```

### Running Locally

```bash
# Run directly without installing
cargo run -- 8080

# Run with verbose flag
cargo run -- 8080 --verbose
```

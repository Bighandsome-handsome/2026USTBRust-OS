# USTB Operating System Course - Rustlings

This project contains some exercises to get you used to reading and writing Rust code, designed for the USTB Operating System course. It is based on [Rustlings](https://github.com/rust-lang/rustlings).

## Installation

First, ensure you have Rust installed. Then, install the local `rustlings` tool:

```bash
cargo install --force --path .
```

## Usage

> [!NOTE]
> **Delete the line** `// I AM NOT DONE` to mark it as complete.

Run the following command to start the **interactive** "watch" mode. It will automatically re-run tests when you save files.

```bash
rustlings watch
```

Other commands:

- **Check all exercises**: `rustlings verify`
- **Run a specific exercise**: `rustlings run <name>`
- **Get a hint**: `rustlings hint <name>`

## Judge

Please execute following commands:

```shell
cd /path/to/ustb-os-rustlings
tar -czf ../ustb-os-rustlings.tar.gz .
```

Then submit `ustb-os-rustlings.tar.gz` to the online judge platform.
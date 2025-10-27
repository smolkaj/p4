# Hacking on x4c

This document provides a guide for developers who want to contribute to the `x4c` codebase. It outlines the development workflow, from building and testing to submitting changes.

## 1. Prerequisites

Before you begin, make sure you have the following installed:

-   **Git:** For cloning the repository.
-   **Rust Toolchain:** The project specifies a particular Rust toolchain version. It is highly recommended to use `rustup` to install and manage it. The required version is defined in the `rust-toolchain.toml` file.

    ```sh
    # rustup will automatically install the correct version
    # when you `cd` into the project directory.
    rustup install
    ```

## 2. Getting Started

First, clone the repository to your local machine:

```sh
git clone https://github.com/oxidecomputer/p4.git
cd p4
```

This project is a Cargo workspace. You can build all the crates within it by running:

```sh
cargo build
```

## 3. Building the `x4c` Compiler

To build the main `x4c` command-line tool, you can use the following command. For development, a debug build is sufficient.

```sh
# Build the x4c binary
cargo build --bin x4c
```

For better performance, you may want to create a release build:

```sh
cargo build --release --bin x4c
```

The resulting executable will be located at `target/debug/x4c` or `target/release/x4c`.

## 4. Installing the `x4c` Compiler

If you want to have `x4c` available in your `PATH`, you can install it using `cargo`:

```sh
cargo install --path x4c
```

This will compile and install the `x4c` binary to your Cargo bin directory (e.g., `~/.cargo/bin/x4c`), making it accessible from anywhere in your terminal.

## 5. Running Tests

The project has a comprehensive test suite in the `test` crate. To run all tests for the workspace and ensure that your changes haven't introduced any regressions, use:

```sh
cargo test
```

Before submitting any changes, please make sure all tests pass.

## 6. Using the `x4c` CLI

Once you have built the compiler, you can use it to compile P4 programs. The `p4/examples/` directory contains several sample P4 files you can use for testing.

To compile a P4 file (e.g., `codegen/core.p4`) into a Rust file, run:

```sh
# Using a debug build
./target/debug/x4c p4/examples/codegen/core.p4
```

This will generate an `out.rs` file in the root of the project. You can specify a different output path using the `-o` flag:

```sh
./target/debug/x4c p4/examples/codegen/core.p4 -o my_pipeline.rs
```

For more options, you can consult the help menu:

```sh
./target/debug/x4c --help
```

## 7. Development Workflow

A typical workflow for making a change to the compiler looks like this:

1.  **Make Code Changes:** Modify the source code in the relevant crate (e.g., `p4/` for the frontend, `codegen/rust/` for the backend).

2.  **Format Your Code:** This project uses `rustfmt` to maintain a consistent code style. Before committing, run:

    ```sh
    cargo fmt
    ```

3.  **Build and Test:** Rebuild the compiler and run the test suite to check for any issues.

    ```sh
    cargo build --bin x4c
    cargo test
    ```

4.  **Commit and Push:** Once your changes are working and all tests pass, commit your changes and open a pull request.

## 8. Working on the Documentation

The project documentation is in the `book/` directory and is built using `mdbook`. To work on the book, you'll first need to install `mdbook`:

```sh
cargo install mdbook
```

To build and serve the book locally, run the following command from the `book/text` directory:

```sh
cd book/text
mdbook serve
```

This will start a local web server, and you can view the book in your browser at `http://localhost:3000`.

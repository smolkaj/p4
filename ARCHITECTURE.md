# x4c Architecture

## 1. Overview

`x4c` is a compiler for the P4 programming language, written entirely in Rust. Its primary function is to compile P4 programs into executable Rust code. The generated Rust code implements a generic `Pipeline` trait, which allows it to be integrated into various Rust-based network applications and testing harnesses.

The project provides two main ways to use the compiler:

1.  **CLI Tool (`x4c`):** A command-line interface that takes a P4 file and generates a corresponding Rust file (`out.rs`).
2.  **Procedural Macro (`use_p4!`):** Allows developers to embed P4 code directly within a Rust project, which is then compiled as part of the crate's build process.

The project is structured as a Cargo workspace, containing several interconnected crates that handle different aspects of the compilation, code generation, and runtime support.

## 2. Project Goals

- **Execute P4 Everywhere:** Enable P4 pipeline logic to run in any environment where Rust is supported.
- **High Fidelity Emulation:** Simulate P4 ASICs with enough accuracy to understand pipeline behavior in a larger system context.
- **Performance:** Be capable of handling real-world network traffic.
- **Introspection:** Provide runtime visibility into program execution through dynamic tracing (e.g., DTrace).
- **Prototyping:** Serve as a foundation for prototyping virtual P4-programmable hardware.

## 3. Core Components

The `x4c` workspace is divided into several key crates, each with a distinct responsibility.

### 3.1. `x4c` (CLI Crate)

-   **Location:** `x4c/`
-   **Purpose:** Provides the command-line interface for the compiler. It parses arguments, invokes the compiler frontend and backend, and writes the generated Rust code to an output file. This is the main entry point for users who want to compile P4 code from the command line.

### 3.2. `p4` (Compiler Frontend)

-   **Location:** `p4/`
-   **Purpose:** This is the heart of the compiler's frontend. It is responsible for processing the raw P4 source code.
-   **Sub-components:**
    -   **Preprocessor:** Handles directives like `#include`.
    -   **Lexer:** Converts P4 source code into a stream of tokens.
    -   **Parser:** Consumes the token stream to build an Abstract Syntax Tree (AST).
    -   **Type Checker (`check.rs`):** Traverses the AST to perform semantic analysis and type checking, ensuring the P4 code is valid.
    -   **HLIR (`hlir.rs`):** Generates a High-Level Intermediate Representation from the AST, which is more suitable for code generation.

### 3.3. `codegen/rust` (Rust Backend)

-   **Location:** `codegen/rust/`
-   **Purpose:** This crate is the compiler's backend. It takes the AST and HLIR from the `p4` crate and generates equivalent Rust code.
-   **Implementation:** It is broken down into modules that handle different P4 language constructs (parsers, controls, headers, etc.). It heavily utilizes the `quote` and `syn` crates to construct the Rust code programmatically.

### 3.4. `lang/p4rs` (Runtime Support Library)

-   **Location:** `lang/p4rs/`
-   **Purpose:** A crucial support library that provides the necessary traits, data structures, and functions required by the Rust code that `codegen/rust` generates. It acts as a runtime layer, offering common functionalities like packet manipulation, table lookups, and checksum calculations. All generated code depends on this crate.

### 3.5. `lang/p4-macro` (Procedural Macro)

-   **Location:** `lang/p4-macro/`
-   **Purpose:** Provides the `use_p4!` procedural macro. This allows for a more integrated developer experience, where P4 code can be written inline or included from a file directly into a Rust crate. The macro invokes the compiler toolchain at build time.

### 3.6. `test` (Integration Testing)

-   **Location:** `test/`
-   **Purpose:** A dedicated crate for integration tests. It contains a collection of P4 programs and corresponding Rust test harnesses that compile the P4 code and verify the behavior of the generated pipelines. This ensures that the compiler and the generated code are correct.

### 3.7. `book` (Documentation)

-   **Location:** `book/`
-   **Purpose:** Contains the source for the `x4c` book, which is the official user documentation for the project, built using `mdbook`.

## 4. Compilation Flow

The compilation process, whether initiated from the CLI or the macro, follows these general steps:

1.  **Input:** The compiler takes a P4 source file as input.
2.  **Frontend Processing (`p4` crate):**
    -   The source code is preprocessed, lexed, and parsed into an AST.
    -   The AST is type-checked for semantic correctness.
    -   An HLIR is generated from the validated AST.
3.  **Backend Code Generation (`codegen/rust` crate):**
    -   The backend traverses the AST and HLIR.
    -   It generates corresponding Rust code, representing the P4 constructs (headers, parsers, control blocks, tables).
    -   The generated code is designed to work with the `p4rs` runtime library.
4.  **Output:**
    -   If using the `x4c` CLI, a `.rs` file is produced.
    -   If using the `use_p4!` macro, the generated Rust code is injected directly into the crate being compiled.

The final product is a Rust module that can be used to instantiate and run the P4-defined packet processing pipeline.

## 5. Directory Structure Summary

-   `p4/`: Compiler frontend (parser, type checker).
-   `codegen/rust/`: Rust code generation backend.
-   `x4c/`: CLI tool.
-   `lang/`: Language-specific crates.
    -   `p4rs/`: Rust runtime support library for generated code.
    -   `p4-macro/`: Procedural macro for `use_p4!`.
-   `test/`: Integration tests.
-   `book/`: Project documentation.
-   `dtrace/`: DTrace scripts for dynamic analysis.
-   `.github/`: CI/CD workflows and configuration.

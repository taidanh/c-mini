# C-Mini Compiler

## Introduction
C-Mini Compiler is a lightweight, efficient compiler for a subset of C, designed to provide fast compilation for small to medium-sized projects. The scope of the compiler extends from the front-end, hand-rolled lexer, parser, and intermediate code generator. Local value numbering optimizes the custom three-address code intermediate representation (IR) before it gets sent to the C++ IR->LLVM->executable backend. 

### Original Version
This project is a rewriting of the same compiler in Python with a C++ addon that makes the code fully runable. This code is private because it is part of a class, so to respect the class please reach out to view the code.

## Getting Started
### Prerequisites
- Rust: Ensure you have the latest version of Rust installed on your machine. Visit [Rust's official site](https://www.rust-lang.org/) for installation instructions.

> **Note:** This project relies on some private code (due to being used in a class) so the C++ portion of the code isn't provided. The project will (once completed) still compile down to a custom IR that mimics a simplified LLVM.

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/taidanh/c-mini
   ```
2. Build:
   ```bash
   cargo build --release
   ```

# 🦀 Najm

A minimal C compiler written in [Rust](https://www.rust-lang.org/).  
This project explores core concepts of compilation including tokenization, parsing, and code generation — all implemented from scratch.

## 🚧 Current Status

This compiler supports a **subset of the C language**, including:

- Variable declarations (e.g. `int x = 5;`)
- Basic arithmetic expressions
- Control flow (`if`, `while`)
- Function definitions and calls
- Return statements

More features will be added incrementally.

## 🎯 Goals

- Learn and demystify compiler construction
- Build a working compiler that can parse, analyze, and generate code for a subset of C
- Keep the architecture clean and extensible

## 📦 Building

You’ll need [Rust](https://www.rust-lang.org/tools/install) installed. Then:

```bash
git clone https://github.com/sualah/najm.git
cd najm
cargo build --release
./najm example.c
# Project Euler Rust Template

A template git repository for solving Project Euler Problems in Rust

**Do not import as a library**, the `build.rs` script used to make this project
work makes it impossible to use as a library, if you want acces to the
hashes/problem descriptions the check out
[Euler Offline](https://github.com/davidcorbin/euler-offline) (source data), and
[Project Euler Manager](https://github.com/salameme/project-euler-manager) the
Rust API I wrote to interface with the data in Euler Offline.

## Usage

1. Make sure you have a working rust toolchain.
2. Use the GitHub interface to generate a repo from this template (or run `cargo generate gh:salameme/project-euler-rust-template`)
3. Use `cargo run -- create PROBLEM_NUMBER` to generate a template for Problem `PROBLEM_NUMBER` this will create a file `src/solution/sol_PROBLEM_NUMBER.rs` which you can edit
4. Use `cargo run -- solve PROBLEM_NUMBER` to compile and run the solution

## Tips and Tricks

- `cargo run -- solve` will run all available solutions
- `cargo run -- solve --time` will run and time all available solutions
- You can use `cargo-watch` to compile and run your solutions while writing them

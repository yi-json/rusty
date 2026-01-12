# Rusty

Hello future me and other snooping developers.

This is a personal reference for my Rust journey. You might learn a thing or two.

## Getting Started

### Creating a Project with Cargo

```bash
cargo new <project_name>
cd project_name
```

### Intializing a Project with Cargo

```bash
cargo init <project_name>
cd project_name
```

### Running a Project with Cargo

```bash
cargo run
```

### Checking a Project Without Producing an Executable

```bash
cargo check
```

Why? `cargo check` is faster than `cargo run` because it simply provides a compiler check without building an executable. Rustaceans run cargo check periodically to catch errors early.

### Building for Release

```bash
cargo build --release
```

Why? `cargo build --release` builds the project with optimizations enabled. Rustaceans run cargo build --release to build a release version of their project.

### Updating a Project with Cargo

```bash
cargo update
```

Why? `cargo update` updates the dependencies in the `Cargo.toml` file. It ignores the previous `Cargo.lock` file and downloads the latest version of the dependencies.

## Variables

Variables and references are immutable by default. To make a variable mutable, use the `mut` keyword.

## Appendix

Crate

- Collection of Rust source code files

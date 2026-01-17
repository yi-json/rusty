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

## Common Programming Concepts

### Constants and Variables

Constants are immutable by default. To make a constant mutable, use the `mut` keyword. Constants are ALWAYS immutable, and you declare them using the `const` keyword.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Shadowing

Declaring a new variable with the same name as an existing variable is called **shadowing**.

If we do the following, we get a compile error:

```rust
let x = 5;
x = 5;
```

By using `let`, we can perform transformations on a value but have the variable be immutable after those transformations are complete.

```rust
let x = 5;
let x = x + 1;
{
    let x = x * 2;
    println!("The value of x in the inner scope is: {}", x);
}
println!("The value of x is: {}", x);
```

```bash
The value of x in the inner scope is: 12
The value of x is: 6
```

## Data Types

### Compound Types

#### Arrays

Fixed size list of the same type. Type is specified as `<type; size>`.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b = [3; 5]; // [3, 3, 3, 3, 3]
```

## Appendix

Crate

- Collection of Rust source code files

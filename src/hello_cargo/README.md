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

### Running a Project in `src/bin`

```bash
cargo run --bin <bin_name>
```

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

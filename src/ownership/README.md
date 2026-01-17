## Ownernship

Set of rules that allow Rust to make memory safety guarantees without a garbage collector. Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

Rules:

- Each value in Rust has a _owner_.
- There can be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

#### The String Type

Made up of three parts:

- A pointer to the memory that holds the contents of the string
- A length - how much memory, in bytes, the contents of the `String` are currently using
- A capacity - total amount of memory, in bytes, that the `String` has received from the allocator

String literals are imutable, so it's not always convenient. For ex, what if we want to take user input and store it? You can create a `String` from a string literal using the `from` function. You can also expand it by using the `push_str` method.

```rust
let s = String::from("hello");
s.push_str(", world!");
println!("{}", s);
```

When we call `String::from`, we request the memroy it needs from the memory allocator. Once the memory (or owner in this case) goes out of scope, it will be dropped by Rust calling the `drop` function.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

When we assign `s1` to `s2`, the `String` data is copied. This means that `s2` now owns the memory, and `s1` is no longer valid. If we try to use `s1` after `s2`, we'll get a compile error.

![copying_str](../assets/copying_str.svg)

```rust
println!("{}", s1);
```

```bash
error[E0382]: borrow of moved value: `s1`
```

To perform a deep copy, we can use the `clone` method.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

#### Ownership and Functions

When we pass a value to a function, the ownership is transferred to the function. The value is no longer valid after the function call. This mimics th same behavior when we assign a variable to another variable.

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

```bash
error[E0382]: borrow of moved value: `s`
```

### References and Borrowing

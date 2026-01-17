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

![copying_str](../../assets/copying_str.svg)

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

The rules of references are:

- At any given time, you can have _either_ one mutable reference or any number of immutable references.
- References must always be valid.
- References do not have ownership.

Take the following code:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
```

The issue is that we have to return the `String` to the calling function so that we can still use the `String` after the call to `calculate_length`.

Instead, we can use a reference to the `String` value.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The `&s1` is a reference to `s1` and does NOT take ownership of the value. Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.

We call the action of creating a reference _borrowing_. If a person owns something, you can borrow it from them. When you're done, you have to give it back because you don't own it.

Basically, references are immutable by default. If we want to mutate the value, we can use a mutable reference.

#### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

We made the following changes:

- `s` is now a mutable reference (`&mut s`)
- `change` is now a function that takes a mutable reference (`fn change(s: &mut String)` and `change(&mut s)`)

### Slices

#### String Slices

The following are equivalent:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

// and
let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

#### Array Slices

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

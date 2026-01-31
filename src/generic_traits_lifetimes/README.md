# Generic Types, Traits, and Lifetimes

## Generic Data Types

We use generics to create definitions for items like structs or enums.

### In Function Definitions

```rs
fn largest<T>(list: &[T]) -> T {}
```

Note that in [largest.rs](src/bin/largest.rs), we use the `Ord` trait bound to ensure that the type `T` implements the `Ord` trait, which is required for the `max` method to work.

So the updated function signature is:

```rs
fn largest<T: std::cmp::Ord>(list: &[T]) -> &T {}
```

### In Struct Definitions

For structs, it's similar to functions. We can also use as many generic type parameters in a definition as you want.

Observe the code in [point.rs](src/bin/point.rs).

```rs
struct Point<T, U> {
    x: T,
    y: U,
}
```

### In Enum Definitions

Some of the most known enums are `Option` and `Result` that use generics.

```rs
enum Option<T> {
    Some(T),
    None,
}
```

```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### In Method Definitions

We can implement methods on structs using generics

```rs
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

We can also specifcy constraints on generic types when defining methods

```rs
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### Monomorphization: Performance with Generics

When we use generics, Rust performs _monomorphization_ to turn generic code into specific code by substituting the generic type with the concrete type. This doesn't run any slower than non-generic code.

## Defining Shared Behavior with Traits <-> Interfaces

A _trait_ defines the functionality a particular has and can share with other types. It's similar to an interface in other languages.

## Validating References with Lifetimes

_Lifetimes_ are a way to ensure that references are valid for as long as we need them to be.

Every reference in Rust has a _lifetime_, which is the scope for which that reference is valid. Life types, lifetimes are implicit and inferred.

### The Borrow Checker

The _Borrow Checker_ compares scopes to determine whether all borrows are valid.

### In Function Signatures

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
```

This signature tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as `'a`. The function returns a string slice that also lasts at least as long as `'a`.

Think of it like a joint bank account. The account is only active as long as both people are alive. If one person dies, the account is closed.

Now, what's the difference between the above snippet and this?

```rs
fn longest<'a>(x: &'a str, y: &'b str) -> &'a str {}
```

Here, we're only telling the compiler that the return value is only tied to `'a`. However, we wouldn't use this function signature because `y` is a temporary string that gets dropped sooner than `x`, which may cause a dangling reference.

### In Struct Definitions

```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

This struct just means that an instance of `ImportantExcerpt` can't outlive the reference it holds in its `part` field.

### Lifetime Elison Rules

1. Each parameter that is a reference gets its own lifetime parameter.
   - If there are > 1 reference parameters, you must specify which lifetime the return reference will have.
   - For ex: `fn longest<'a>(x: &'a str, y: &'b str)` can have a return reference of either `-> &'a str` or `-> &'b str`.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.

# Generic Types, Traits, and Lifetimes

## Generic Data Types

We use generics to create definitions for items like structs or enums.

### In Function Definitions

Let's say we want to make a generic function for finding the largest number in a list. We can declare it as follows:

```rs
fn largest<T>(list: &[T]) -> &T {}
```

Note that in [largest.rs](src/bin/largest.rs), we use the `Ord` trait bound to ensure that the type `T` implements the `Ord` trait, which is required for the `max` method to work.

So the updated function signature is:

```rs
fn largest<T: std::cmp::Ord>(list: &[T]) -> &T {}
```

### In Struct Definitions

For structs, it's similar to functions. We can also use as many generic type parameters in a definition as you want.

Observe the code in [struct_generic.rs](src/bin/struct_generic.rs).

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

Full code in [method_generic.rs](src/bin/method_generic.rs)

### Monomorphization: Performance with Generics

When we use generics, Rust performs _monomorphization_ to turn generic code into specific code by substituting the generic type with the concrete type. This **does not run any slower** than non-generic code.

## Defining Shared Behavior with Traits

A _trait_ defines the functionality a particular type has and can share with other types. It's similar to an _interface_ in other languages.

### Defining a Trait

A type's behavior is based on the methods we can call. We use trait definitions to group method signatures together to define a set of behaviors.

For ex, we can have multiple structs that holds various kinds and amounts of text like `NewsArticle`, `SocialPost`, etc. We want to make a media aggregator library crate named `aggregator` that can display summaries of data that might be stored in a `NewsArticle` or `SocialPost` instance.

### Implementing a Trait on a Type

We define the following trait as follows. Full code is in [aggregator.rs](src/aggregator.rs):

```rs
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

The syntax for implementing a trait is similar to implementing regular methods. The format is `impl <TraitName> in <StructName>` like this:

```rs
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Now, in [main.rs](src/main.rs), we declare `aggregator` as a **module** so we can access the code in `aggregator.rs`. Let's say we want to make a `SocialPost`

```rs
mod aggregator
use aggregator::{SocialPost, Summary}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
    };

    println!("1 new tweet: {}", post.summarize());
}
```

### The Orphan Rule

This is essentially Rust's way of preventing "naming collisions" and chaos in your code. It ensures that there is only **one** clear implementation of a trait for a specific type. To implement a trait for a type, you, i.e your **crate**, must own either the **Trait** or the **Type**.

If you own neither, then you are an **Orphan**, i.e no parantage over either side, which Rust forbids the implementation.

By doing so, we enforce __coherence__ - there is only ever one "correct" implementation of a trait for a type in any given program. This prevents you from implementing a foreign trait on a foreign type, i.e `impl ExternalTrait for ExternalType`

Observe the example in [orphan.rs](src/orphan.rs):

```rs
use aggregator::SocialPost;
use std::fmt;

// ERROR: This is an Orphan Rule violation!
impl fmt::Display for SocialPost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Post by {}", self.username)
    }
}
```

- Do I own the trait `Display`? No, `std` owns it
- Do I own the type `SocialPost`? No, aggregator owns it

Note that since we did not include `mod aggregator` at the top, we acknowledge that `SocialPost` lives in a different package

Since the answer is No to both, you are an "orphan" to this implementation.

### Using Traits as Parameters

We can use traits to define functions that accept many different types. For now, let's start with just accepting the `Summary` type. Look in [aggregator.rs](src/aggregator.rs):

#### Trait Bound Syntax

A trait bound is a constraint placed on a generic type parameter that forces it to implement a specific set of traits. It restricts the allowed types to those that conform to the bound, enabling you to call methods defined in that trait on the generic type within your function or struct.

```rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());

    // in this body, we can call any methods on item that come from the Summary trait such as summarize()
}
```

Then to use in [main.rs](src/main.rs), we do:

```rs
mod aggregator;
use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
    };

    aggregator::notify(&post);
}
```

Note that we don't need to add `notify` in the `use aggregator` because it is declared as `pub`!

#### Multiple Trait Bounds with the `+` Syntax

We can also specify more than one trait bound like so:

```rs
use std::fmt::Display;
pub fn notify_display(item: &(impl Summary + Display))
```

Now the `item` must implement BOTH `Summary` and `Display`

#### Clearer Trait Bounds with `where` Clauses

Too many trait bounds has its downsides. We can rewrite multiple trait bounds using the `where` clause:

```rs
use std::fmt::{Display, Debug};
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}
```


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

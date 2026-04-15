# Object-Oriented Programming

## Characteristics of Object-Oriented Languages

`pub` is used to declare something as public, and by default, everything else is private.

## Using Trait Objects to Abstract Over Shared Behavior

### Defining a Trait for Common Behavior

A **trait object** points to two things:
- an instance of a type implementing our specified trait
- a table used to look up trait methods on that type on runtime. It basically says, if this is a `Draft` trait, then here is the memory address for the `request_review` function. If it's `Published`, use this other function.

In Rust, recall that we do **not** refer to structs and enums as "objects" to distinguish them from other languages. Unlike objects in other languages, we **cannot** store data to a trait object.

__So what's the purpose of a trait object?__

Allow abstraction across common behavior. Different structs that implement the `State` trait might have different sizes:
- `struct Draft{}` takes 0 bytes
- A hypothetical `struct Rejected { reason: String }` might take 24 bytes


We create one by specifying some point of pointer, such as a reference of a `Box<T>` smart pointer, then add `dyn <relevant_trait>` like this:

```rs
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

The vector is of type `Box<dyn Draw>`, which is a trait object; a stand-in for any type inside a `Box` that implements the `Draw` trait.

### Implementing the Trait

By specifying `Box<dyn Draw>` as the type of values in the `components` vector, we've defined `Screen` to need values that we can call the `draw` method on.

### Performing Dynamic Dispatch

Normally, Rust uses **static dispatch**. When you call a function, the compiler knows exactly which code to run. But with the State Pattern, the `Post` doesn't know if the current state is a `Draft` or `Published` until the program is actually running.

The `dyn` keyword is actually short for **dynamic**. It tells the compiler: *"I don't know the concrete type of this object right now, but we'll figure out which method to call at runtime"*

#### Static Dispatch

Recall in [Generic, Traits, and Lifetimes](/src/generic_traits_lifetimes/README.md), we discussed the **monomorphization process** performed on generics by the compiler. The code that results from this is *static dispatch*, which is when the compile knows what method you're calling in __compile time__.

#### Dynamic Dispatch

Used when we use **trait objects**. The compiler doesn't know all the types that might be used with the code that's using trait objects until at runtime where Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a *runtime cost* that static dispatch doesn't have.

## Implementing an Object-Oriented Design Pattern

### The State Pattern

We define a set of states a value can have internally. The states are represented by a set of **state objects**, and the value's behavior changes based on its state.

We will compare the traditional OOP style vs Rust-natural by doing a Blog Post workflow:
- A blog post starts as an empty draft.
- When the draft is done, a review of the post is requested.
- When the post is approved, it gets published.
- Only published blog posts return content to print so that unapproved posts can’t accidentally be published.

### Attempting Traditional Object-Oriented Style

```rs
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }
}

trait State {}
struct Draft {}
impl State for Draft {}
```

When we create a new `Post`, we set the `state` field to a `Some` value that holds a `Box`, which points to a `Draft` struct on the heap. This ensures that whenever we make a new `Post`, it will start as a `Draft`.

Note that since posts are initially in the `Draft` state, the content is always empty.

__Why is `add_text()` not part of the State Pattern?__

The behavior doesn't depend on the state the post is in, so we omit it from the `trait State`.


### Requesting a Review, Which Changes the Post's State

When we request review of a post, it should change its state from `Draft` to `PendingReview`.

```rs
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

### The Rust Way: Encoding States and Behavior as Types

Instead of having a `content` method on a draft post that returns an empty string, we make it so that draft posts **don't** have the `content` method at all.

```rs
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

In this approach, we don't need to encapsulate the transformations between states within the `Post` implementation. Invalid states are now impossible due to the type system that occurs at compile time. This ensures that certain bugs, such as displaying the content of an unpublished post will be discovered without having to run the program.
## Structs

We use the following struct for the examples:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### Printing Fields in Structs

```rust
println!("user1.active={0}", user1.active);
```

### Printing the Entire Struct

With structs, it's not entirely clear how to print the entire struct. We need to implement the `Debug` trait for the struct:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rec is {:#?}", rec);
}
```

### Creating Instances with Struct Update Syntax

Let's say you want to create an instance of a struct but leveraging the data of an existing struct:

Note that since we are modifying the struct after it was created, we have to mark the entire struct instance using `mut`.

```rust
let mut user1 = User {
        active: true,
        username: String::from("someone123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

user1.email = String::from("anotheremail@example.com");

let user2 = User {
    email: String::from("another@example.com"),
    ..user1
}
```

Couple of things to note:

- Note that `..<name_of_instance>` has to come in last
- Both `active` and `sign_in_count` implement the `Copy` trait, so they are still valid
- We can still use `user1.email` since it hasn't been moved
- We cannot use `user1.username` since it has been moved

### Creating Different Types with Tuple Structs

To access the the ith element of a tuple struct, we use the dot operator `.` instead of doing brackets like in an array `[]`:

```rs
struct Color(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    println!("{}", black.0);
}
```

### Methods - Functions in Structs

To define a function within the context of a struct, we start an `impl` (implementation) block:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

Note that `&self` is required as a first parameter. It's short for `self: &Self`, meaning it takes an immutable reference to the struct reference. If we had just done `self`, the method would have taken ownership of the instance, consuming it.


### Associated Functions

Functions defined in an `impl` block.

We can define associated functions that don't take `self` as a parameter (and thus aren't methods) because they don't have access to the instance data

Often used for constructor functions that will return a new instance of a struct - often called `new`

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

We then call it using the notation `<className>::<functionName>`:

```rs
fn main() {
    let square = Rectangle::square(4);
    println!("The size of square is: {}", square.width);
    println!("The area of square is: {}", square.area());
}
```
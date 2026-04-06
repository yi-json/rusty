## Common Collections

### Vectors

For official API Documentation: check [https://doc.rust-lang.org/std/vec/struct.Vec.html](https://doc.rust-lang.org/std/vec/struct.Vec.html)

Can only store values of the same type.

You can create an empty vector or with initial values:

```rs
fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
}
```

Note that Rust inferes the type annotation, so the `: Vec<T>` is optional, but it's good practice.

#### Reading Elements of Vectors

There are two ways: using `&` and `[]`, and using `get()`

```rs
let third: &i32 = &v2[2];
println!("The third element is {}", third);

let third: Option<&i32> = v2.get(2);
match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element"),
}
```

You typically use `get()` for user input (like accidentally entering a number too large) or when accessing an element beyond the range of the vector happens occasionally.

#### Iterating Elements of Vectors

Here's mainly for reading:

```rs
// iterating over values in a vector
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

And this is for mutating using mutable referenes, without creating a new vector:

Note that `x: &mut i32`.
```rs
// iterating and mutating values; adding 50 to each value
let mut v = vec![100, 32, 57];
for x in &mut v {
    *x += 50;
    println!("Before mutation: {}, After mutation: {}", *x - 50, x);
}
```

However, if `v` is a `&mut Vec<i32>`, but we don't want to mutate the values themselves, we do this:

```rs
fn get_median_mode(v: &mut Vec<i32>) -> (f64, i32) {
    let mut freq = HashMap::new();
    for &x in v.iter() {
        let cnt = freq.entry(x).or_insert(0);
        *cnt += 1;
    }
    // rest of logic...
}
```

#### Representing a Stack using `Vec<T>`

Simiar to Python, the idiomatic way to represent a stack is by using a `Vec`. 

You can see what you can do with it by going to [src/bin/stack.rs](src/bin/stack.rs)

TL:DR, you just need `last()` for the top of the stack, and `pop()`


#### Sorting

**Always use `.sort_unstable()` for Primitives**

If you are sorting a `Vec<i32>`, `Vec<f64>`, or `Vec<char>`, stability is meaningless. One 5 is exactly the same as another 5. There is no "original order" worth preserving because you can't tell the difference between the two anyway.
- Sorting numbers, scores, or basic counts

**Use `.sort()` when "Equal" Isn't "Identical"**

Stability matters when you are sorting complex data (like Structs) by a specific field, but you want to keep a secondary order intact.

Example:
- Imagine you have a list of employees already sorted by First Name. Now you want to sort them by Department.
= Stable Sort: Within the "Engineering" department, employees will still be in alphabetical order by their First Name.
- Unstable Sort: The "Engineering" department will be grouped together, but the First Names will be scrambled into a random order.


#### Accessing the Median/Middle of a Vector

You can write it explicitly:
```rs
let n = v.len();
let mid = n / 2;
let median = if n % 2 == 0 {
    (v[mid - 1] as f64 + v[mid] as f64) / 2.0
} else {
    v[mid] as f64
};
```

Or use a crate:

In `Cargo.toml`
```toml
[dependencies]
statistical = "1.0.0"
```

Then on your `.rs` file:
```rs
use statistical::median;

fn main() {
    let numbers = vec![9, 1, 5, 3, 7];
    
    // One line, no mutable references or sorting required on your end
    println!("The median is: {}", median(&numbers)); 
}
```

### Hash Maps

All of the keys must have the same type, and all of the values must have the same type.

#### Reading Values in Hash Maps

```rs
let key = String::from("key");
let val = hs.get(&key).copied().unwrap_or(0);
```

Here, `get()` returns an `Option<&V>`. if there's no value for that key in the hashmap, `get` returns `None`. Else, we handle `Option` by calling `copied` to get `Option<i32>` rather than an `Option<&i32>`. Then `unwrap_or` to set `score` to zero if `scores` doesn't have an entry for the key.

#### Managing Ownership in Hash Maps

For types that implement the `Copy` trait -- i8, i32, u64, usize, f32, f64, bool, char, tuples and arrays that only contain elements that implement `Copy`, immutable references -- values are copied into the hashmap.

For owned values/types that use move semantics -- String, Collections (Vec<T>, Hashmap<K, V>), Smart Pointers, System Resources (File, network sockets, locks), mutable references, structs/enums -- values will be moved and the hashmaop will be the owner of those values

```rs
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

#### Updating The Hashmap

##### Overwriting
```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");
```

##### Adding a Key and Value Only if a Key Isn't Present

Let's say we want to check whether a particular key already exists in the hash map with the value and then take the following actions:
- If key in hashmap, the existing value should remain the way it is
- Else, insert it and a value for it

To do this, we use `entry()`, which returns an enum `Entry`. 

##### Finding Max Value of Hashmap

```rs
let mut scores = HashMap::new();
scores.insert("Alice", 50);
scores.insert("Bob", 99);

// We use (&top_name, &top_score) to destructure the references right away
let (&top_name, &top_score) = scores
    .iter()
    .max_by_key(|&(_, v)| v)
    .expect("The map was empty!");

println!("Stored! Top student is {} with {}", top_name, top_score);
```

Why does the closure look like `|(_, v) | v`?
- It uses pattern matching to ignore the key via `_`, and extract a reference to the value `v`
- It then returns `v`, telling `max_by_key()` to use that specific value for its comparison sorting
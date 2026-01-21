## Enums

Gives you a way of saying a value is one of a possible set of values

There are two advantages to using enums over structs

1. Instead of putting enums in structs, we can put data directly in each enum variant:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

2. Each variant can have different types and amounts of associated data

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

Why is having `Option<T>` better than having null?
- `Option<T>` and `T` are different types; the compiler won't let us use an `Option<T>` as if it were a valid value
- You have to convert an `Option<T>` to a `T` before you can perform `T` operations with it
- This forces you to handle the `None` case explicitly

### The `match` Control Flow Construct

Allows you to compare a value against a series of patterns and execute code based on which pattern matches

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### The `Option<T> match` Pattern

Used to get the inner `T` value from an `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Note that matches are exhaustive; you must cover all possible cases

There are a handful ways to handle missing cases:

1. Catch-all using `other`

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```

2. `_` is a catch-all pattern that matches any value

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```
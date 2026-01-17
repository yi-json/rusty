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

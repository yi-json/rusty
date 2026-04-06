fn main() {
    let mut stack = Vec::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    // peek at top without removing it
    if let Some(top) = stack.last() {
        println!("Top element: {}", top); // prints 30
    }

    // pop elements off the stack
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}

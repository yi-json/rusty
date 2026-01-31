struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_int = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 5, y: 4.0 };
}

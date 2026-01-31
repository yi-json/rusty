fn largest<T: std::cmp::Ord>(list: &[T]) -> &T {
    match list.iter().max() {
        Some(max_val) => max_val,
        None => panic!("The vector is empty!"),
    }
}

fn main() {
    let number_list = vec![30, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));
}

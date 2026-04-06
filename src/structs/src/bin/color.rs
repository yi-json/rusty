struct Color(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);

    println!("{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
}

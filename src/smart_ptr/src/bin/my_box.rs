use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn add_world(s: &mut String) {
    s.push_str(" World!");
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // deref coercion makes it possible to call `hello` with a reference to a value of type
    // MyBox<String>

    // in short, deref can turn &MyBox<String> -> &String
    let m = MyBox(String::from("Rust"));
    hello(&m);

    // mutable deref coercion
    let mut my_box = MyBox(String::from("Hello"));
    add_world(&mut my_box);
    assert_eq!(*my_box, "Hello World!");
}

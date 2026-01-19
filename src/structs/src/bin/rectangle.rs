/*
 * Write a function that calculates the area of a rectangle.
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle is {} square pixels", rec1.area());
    println!("rec={:#?}", rec1);

    if rec1.width() {
        println!("The rectangle has a nonzero width; it is {}", rec1.width);
    }

    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));
}

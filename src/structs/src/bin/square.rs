struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 2,
        height: 2,
    };
    let rec2 = Rectangle {
        width: 3,
        height: 4,
    };

    assert_eq!(rec1.can_hold(&rec2), false);
    assert_eq!(rec2.can_hold(&rec1), true);

    let square = Rectangle::square(4);
    println!("The size of square is: {}", square.width);
    println!("The area of square is: {}", square.area());
}

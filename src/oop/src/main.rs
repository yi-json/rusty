mod gui;
use gui::{Draw, Screen};

mod blog;
use blog::Post as BlogPost;

mod enc;
use enc::Post as EncPost;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    // gui.rs
    let screen = Screen {
        components: vec![Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        })],
    };

    screen.run();

    // blog.rs
    let mut post = BlogPost::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // enc.rs: the rust way of doing OOP
    let mut post = EncPost::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

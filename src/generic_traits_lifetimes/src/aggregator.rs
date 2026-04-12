use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // here, we've omitted an impl for summary() so we can use the default impl
}

// using traits as parameters and trait bounds
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());

    // in this body, we can call any methods on item that come from the Summary trait such as
    // summarize()
}

pub fn notify_display(item: &(impl Summary + Display)) {
    println!(
        "Breaking news for Summary and Display! {}",
        item.summarize()
    );
}

// multiple trait bounds via where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("json"),
        email: String::from("json@google.com"),
        sign_in_count: 0,
        active: true,
    };

    let user2 = User {
        username: String::from("yaml"),
        email: String::from("yaml@snowflake.com"),
        ..user1
    };

    println!("{}", user1.username); // compiles! it prints "json"
}

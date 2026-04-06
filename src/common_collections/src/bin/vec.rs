fn main() {
    let v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v2.push(4);
    v2.push(5);

    // reading elements in vectors
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterating and mutating values; adding 50 to each value
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("Before mutation: {}, After mutation: {}", *i - 50, i);
    }

    // using enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
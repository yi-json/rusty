use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // inserting values in hs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // reading values in hs
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // alternative way to reading values
    if let Some(score) = scores.get(&String::from("Blue")) {
        println!("Blue score: {}", score);
    }

    // iterating over a hs
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    // overwriting an entry
    println!("score before overwriting: {}", score);
    scores.insert(String::from("Blue"), 20);
    let new_score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("score after overwriting: {}", new_score);

    // adding a key and val only if the key isn't present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // updating value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    // max value of a hashmap
    let (&top_key, &max_freq) = map
        .iter()
        .max_by_key(|&(_, v)| v)
        .expect("The map was empty!");

    println!("max freq of text: {}:{}", top_key, max_freq);
}

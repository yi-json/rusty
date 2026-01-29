use std::time::Duration;

fn main() {
    // 1. First loop runs immediately and blocks the thread
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        std::thread::sleep(Duration::from_millis(500));
    }

    // 2. Second loop only starts after the first one finishes
    for i in 1..5 {
        println!("hi number {i} from the second task!");
        std::thread::sleep(Duration::from_millis(500));
    }
}

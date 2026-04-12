fn largest<T: std::cmp::PartialOrd>(l: &[T]) -> &T {
    let mut largest = &l[0];

    for item in l {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn main() {
    let l = vec![34, 50, 25, 100, 65];
    let l2 = vec!["a", "b", "c"];
    let ans1 = largest(&l);
    let ans2 = largest(&l2);
    println!("The largest number is {}", ans1);
    println!("The largest char in l2 is {}", ans2);
}

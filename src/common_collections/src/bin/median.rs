/* Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list. */

use std::collections::HashMap;

fn get_median_mode(v: &mut Vec<i32>) -> (f64, i32) {
    v.sort_unstable();
    let mut freq = HashMap::new();
    for &x in v.iter() {
        let cnt = freq.entry(x).or_insert(0);
        *cnt += 1;
    }

    // logic for median
    let n = v.len();
    let mid = n / 2;
    let median = if n % 2 == 0 {
        (v[mid - 1] as f64 + v[mid] as f64) / 2.0
    } else {
        v[mid] as f64
    };

    // logic for mode
    let (&mode, _) = freq.iter().max_by_key(|&(_, v)| v).expect("Empty map");

    (median, mode)
}

fn main() {
    let mut v1 = vec![1, 2, 3, 3, 4, 5];
    assert_eq!(get_median_mode(&mut v1), (3.0, 3));
}

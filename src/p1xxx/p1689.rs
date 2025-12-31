use std::ops::Deref;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.as_bytes()
            .iter()
            .reduce(|accum, ch| accum.max(ch))
            .map(|&s| (s - b'0') as i32)
            .unwrap()
    }
}

fn main() {
    println!("Hello World");
}

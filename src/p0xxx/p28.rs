// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 28
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1, |s| s as i32)
    }
}

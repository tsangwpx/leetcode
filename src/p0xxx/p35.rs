fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 35
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        assert!(nums.len() >= 1);

        // bisect left
        let mut a = 0;
        let mut b = nums.len(); // exclusive

        // break loop only if no gap
        while a < b {
            let m = (a + b) / 2;
            let value = nums[m];

            // no equality
            if value < target {
                a = m + 1;
            } else {
                b = m;
            }
        }

        a as i32
    }
}

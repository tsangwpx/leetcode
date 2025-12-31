// Problem 594
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counter = HashMap::<i32, i32>::new();

        nums.iter().for_each(|&s| {
            *counter.entry(s).or_default() += 1;
        });

        let mut max_len = 0;

        for (&key, &count) in counter.iter() {
            // if key - 1 exist, then this key will be also checked when key - 1 is visited
            let key2 = key + 1;

            if let Some(count2) = counter.get(&key2).copied() {
                max_len = max_len.max(count + count2);
            }
        }

        max_len
    }
}

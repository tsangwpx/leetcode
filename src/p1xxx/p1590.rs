// Problem 1590
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        use std::collections::HashMap;

        let sum = nums
            .iter()
            .copied()
            .fold(0i64, |acc, item| (acc + item as i64) % (p as i64));

        if sum == 0 {
            return 0;
        }

        let mut last_occurrences = HashMap::new();
        last_occurrences.insert(0, -1);

        let mut acc = 0;
        let mut width = nums.len();

        // println!("sum {}", sum);

        for (idx, item) in nums.iter().copied().enumerate() {
            acc += item as i64;
            acc %= p as i64;

            let rem = acc;
            let com = (acc - sum).rem_euclid(p as i64);
            if let Some(&idx2) = last_occurrences.get(&(com as i32)) {
                width = width.min(idx + 1 - (idx2 + 1) as usize);
            }

            last_occurrences.insert(rem as i32, idx as i32);
        }

        if width == nums.len() {
            -1
        } else {
            width as i32
        }
    }
}

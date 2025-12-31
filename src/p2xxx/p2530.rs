// Problem 2530
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;

        const DIVISOR: i32 = 3;

        if nums.is_empty() {
            return 0;
        }

        let mut score = 0i64;
        let mut pq = BinaryHeap::from(nums);

        for idx in 0..k {
            let number = pq.pop().unwrap();

            if number == 1 {
                // shortcut
                let remaining = k - idx;
                score += i64::from(remaining);
                break;
            } else {
                score += i64::from(number);
                pq.push((number + DIVISOR - 1) / DIVISOR);
            }
        }

        score
    }
}

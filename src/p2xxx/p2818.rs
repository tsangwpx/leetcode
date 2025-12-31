// Problem 2818
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let maximum = nums.iter().copied().max().unwrap();

        let mut prime_scores = vec![0; maximum as usize + 1];

        for i in 2..prime_scores.len() {
            let is_prime = prime_scores[i] == 0;
            if is_prime {
                for j in (i..=maximum as usize).step_by(i) {
                    prime_scores[j] += 1;
                }
            }
        }

        // println!("{:?}", nums);
        // println!("{:?}", prime_scores);

        let mut left_indices = vec![0usize; nums.len()];
        let mut right_indices = left_indices.clone();
        let mut stack = vec![];

        for idx in 0..nums.len() {
            let number = nums[idx];
            let score = prime_scores[number as usize];
            while let Some((_, prev_score)) = stack.last().copied() {
                if prev_score >= score {
                    break;
                }

                stack.pop();
            }

            left_indices[idx] = if let Some((prev_idx, _)) = stack.last().copied() {
                prev_idx + 1
            } else {
                0
            };

            stack.push((idx, score));
        }

        stack.clear();
        for idx in (0..nums.len()).rev() {
            let number = nums[idx];
            let score = prime_scores[number as usize];
            while let Some((_, next_score)) = stack.last().copied() {
                if next_score > score {
                    break;
                }

                stack.pop();
            }

            right_indices[idx] = if let Some((next_idx, _)) = stack.last().copied() {
                next_idx
            } else {
                nums.len()
            };

            stack.push((idx, score));
        }

        // println!("{:?}", left_indices);
        // println!("{:?}", right_indices);

        let mut heap = BinaryHeap::new();

        for idx in 0..nums.len() {
            let number = nums[idx];
            let left = left_indices[idx];
            let right = right_indices[idx];
            let count = (idx - left + 1) as i64 * (right - idx) as i64;
            heap.push((number, count));
        }

        fn powmod(base: i64, exp: i64) -> i64 {
            // Compute pow(base, exp, MOD) in Python

            let mut pow = base;
            let mut work = exp;
            let mut bit = 1;
            let mut res = 1i64;

            while work != 0 {
                if (work & bit) != 0 {
                    work ^= bit;
                    res *= pow;
                    res %= MOD;
                }

                bit <<= 1;
                pow *= pow;
                pow %= MOD;
            }

            res
        }

        let mut quota = k as i64;
        let mut res = 1i64;
        const MOD: i64 = 10i64.pow(9) + 7;
        while quota > 0 && !heap.is_empty() {
            let Some((number, count)) = heap.pop() else {
                unreachable!();
            };

            let number = i64::from(number);
            let consumed = quota.min(count);
            quota -= consumed;

            res *= powmod(number, consumed);
            res %= MOD;
        }

        res as i32
    }
}

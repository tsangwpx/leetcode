use core::num;

// Problem 2963
impl Solution {
    pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut table = HashMap::<i32, u32>::with_capacity(1024.min(nums.len()));

        for (idx, &number) in nums.iter().enumerate() {
            table.insert(number, idx as u32);
        }

        let mut partitions = 0;
        let mut new_partition_idx = 0;

        for (idx, &number) in nums.iter().enumerate() {
            if idx == new_partition_idx {
                partitions += 1;
            }

            let last_idx = *table.get(&number).unwrap() as usize;
            new_partition_idx = new_partition_idx.max(last_idx + 1);
        }

        // If there are N+1 indivisible paritions, there are N gaps to concat them
        // so A = 2 ** N is the number of possible ways to combine them. (ie. sum of combinations).

        let mut power = partitions - 1;
        let mut ans = 1i64;
        let mut squaring = 2;
        const MOD: i64 = 10i64.pow(9) + 7;
        const BASE: i64 = 2;

        while power != 0 {
            if power & 1 == 1 {
                ans = (ans * squaring) % MOD;
            }
            // println!("{}: {}", power, ans);
            squaring = (squaring * squaring) % MOD;
            power = power >> 1;
        }

        ans as i32
    }
}

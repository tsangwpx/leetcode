// Problem 3583
impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut counter = HashMap::<i32, [i64; 2]>::new();
        let mut res = 0i64;

        for item in nums.iter().copied() {
            let half = if item % 2 == 0 { Some(item / 2) } else { None };
            let double = item * 2;

            // item as nums[k]
            if let Some(half) = half {
                if let Some([_, cj]) = counter.get(&half).copied() {
                    res += cj;
                    res %= MOD;
                }
            }

            // item as nums[j]
            if let Some([ci, _]) = counter.get(&double).copied() {
                let [_, cj] = counter.entry(item).or_default();
                *cj = (*cj + ci) % MOD;
            }

            // item as nums[i]
            let [ci, _] = counter.entry(item).or_default();
            *ci = (*ci + 1) % MOD;
        }

        res as i32
    }
}

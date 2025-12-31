// Problem 2845
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        use std::collections::HashMap;

        /* c1 and c2 are running sum
         * (c2 - c1) % module = k
         * c2 % mod = (k % mod + c1 % mod) % mod
         * Remarks:
         * 1. c2 is acc
         * 2. (c2 + k) % mod is key in table
         * 3. For c1 = 0, c2 % module == k,
         *      we need table[k] = 1  to count this case as initial condition
         */

        let mut table = HashMap::with_capacity(nums.len().min(modulo as usize));
        table.insert(k, 1);

        let mut acc = 0;
        let mut count = 0;
        let mut comp = (acc + k) % modulo;

        for number in nums.iter().copied() {
            if number % modulo == k {
                acc += 1;
                acc %= modulo;
                comp = (acc + k) % modulo;
            }

            count += table.get(&acc).copied().unwrap_or(0) as i64;
            *table.entry(comp).or_default() += 1;
        }

        count
    }
}

// Problem 1498
impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        /*
         * Observations:
         * 1. item positions are not important, because only the min and max of subseq is needed.
         * 2. thus, the array can be rearranged or sorted such that
         *      the min and the max of subsequences are visited systematically.
         * 3. given a sorted array, the problem become a window of two pointers.
         * 4. given a min index, the number of subsequences is the number of max indices,
         *      including the min index itself. it is 2 ** (j - i),
         *      where i and j are two endpoint of the window.
         * 5. powmod() is copied from previous problem.
         */

        assert!(nums.len() >= 1);

        const MOD: i64 = 10i64.pow(9) + 7;

        #[inline]
        fn powmod(base: i64, exp: i64) -> i64 {
            // Compute pow(base, exp, MOD) in Python

            let mut mul = base % MOD;
            let mut bits = exp;
            let mut res = 1i64;

            loop {
                if bits & 1 == 1 {
                    res *= mul;
                    res %= MOD;
                }

                bits >>= 1;

                if bits == 0 {
                    break;
                }

                mul *= mul;
                mul %= MOD;
            }

            res
        }

        nums.sort_unstable();

        let mut right = nums.len() - 1;

        let mut count = 0;

        for (idx, min) in nums.iter().copied().enumerate() {
            while right > idx && min + nums[right] > target {
                right -= 1;
            }

            if min + nums[right] <= target {
                let len = (right + 1 - idx) as i64;
                count += powmod(2, len - 1);
                count %= MOD;
            }
        }

        count as i32
    }
}

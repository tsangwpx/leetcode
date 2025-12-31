// Problem 3392
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for win in nums.windows(3) {
            let &[a, b, c] = win else {
                continue;
            };

            if (a + c) * 2 == b {
                count += 1;
            }
        }

        count
    }
}

// Problem 1863
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;

        for mask in 1..2usize.pow(nums.len() as u32) {
            let mut xor = 0;
            for (idx, item) in nums.iter().copied().enumerate() {
                if (mask >> idx) & 1 == 1 {
                    xor ^= item;
                }
            }

            sum += xor;
        }

        sum
    }
}

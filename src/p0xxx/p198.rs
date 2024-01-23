fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 198
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);

        if nums.len() <= 2 {
            return *nums.iter().max().unwrap();
        }

        // dp array: money at i-th state  if the i-th house is robbed / not robbed.
        // it can be optimized as scalar variables
        let mut dp_robbed = vec![0; nums.len()];
        let mut dp_unrobbed = vec![0; nums.len()];
        dp_robbed[0] = nums[0];

        for (i, &money) in nums.iter().enumerate().skip(1) {
            dp_robbed[i] = money + dp_unrobbed[i - 1];
            dp_unrobbed[i] = dp_robbed[i - 1].max(dp_unrobbed[i - 1]);
        }

        *dp_robbed.last().unwrap().max(dp_unrobbed.last().unwrap())
    }
}

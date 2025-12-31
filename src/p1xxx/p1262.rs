// Problem 1262
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp0 = [0; 3];

        for item in nums.iter().copied() {
            let mut dp1 = dp0;

            for i in 0..3 {
                let sum = dp0[i] + item;
                let j = (sum % 3) as usize;
                dp1[j] = dp1[j].max(sum);
            }

            dp0 = dp1;
        }

        dp0[0]
    }
}

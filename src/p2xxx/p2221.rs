// Problem 2221
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut dp0 = nums;
        let mut dp1 = Vec::with_capacity(dp0.len());

        while dp0.len() >= 2 {
            dp1.clear();
            dp1.extend(dp0.windows(2).map(|s| (s[0] + s[1]) % 10));
            std::mem::swap(&mut dp0, &mut dp1);
        }

        let Some(res) = dp0.pop() else {
            panic!("bad input")
        };

        res
    }
}

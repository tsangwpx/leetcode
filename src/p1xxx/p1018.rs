// Problem 1018
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut res = vec![false; nums.len()];
        let mut rem = 0;
        for (idx, item) in nums.iter().copied().enumerate() {
            rem <<= 1;
            rem += item;
            rem %= 5;
            res[idx] = rem == 0;
        }

        res
    }
}

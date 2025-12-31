// Problem 1920
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];

        for i in 0..nums.len() {
            res[i] = nums[nums[i] as usize];
        }

        res
    }
}

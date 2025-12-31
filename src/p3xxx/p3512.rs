// Problem 3512
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().copied().fold(0, |acc, item| (acc + item) % k)
    }
}

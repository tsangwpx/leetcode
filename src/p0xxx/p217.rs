use std::ffi::FromVecWithNulError;

// Problem 217
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let set = nums.into_iter().collect::<std::collections::HashSet<i32>>();

        len > set.len()
    }
}

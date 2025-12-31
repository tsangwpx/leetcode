// Problem 2016
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = -1;

        for item in nums.iter().copied() {
            if item > min {
                max = max.max(item - min);
            }

            min = min.min(item);
        }

        max
    }
}

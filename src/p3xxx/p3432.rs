// Problem 3432
impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();

        let mut left = 0;
        let mut count = 0;
        for item in nums.iter().take(nums.len().saturating_sub(1)).copied() {
            left += item;
            let right = sum - left;
            let is_even = (left - right) % 2 == 0;
            if is_even {
                count += 1;
            }
        }

        count
    }
}

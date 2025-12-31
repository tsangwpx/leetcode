// Problem 976
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        for k in (2..nums.len()).rev() {
            let i = k - 2;
            let j = k - 1;
            if nums[i] + nums[j] > nums[k] {
                return nums[i] + nums[j] + nums[k];
            }
        }

        0
    }
}

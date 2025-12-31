// Problem 53
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // https://en.wikipedia.org/wiki/Maximum_subarray_problem
        let mut max_here = nums[0];
        let mut maximum = max_here;

        for &number in nums.iter().skip(1) {
            max_here = number + 0i32.max(max_here);
            maximum = maximum.max(max_here);
        }

        maximum
    }
}

// Problem 153
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo < hi {
            let mid = (lo + hi) / 2;

            // equality is impossible because mid != hi and all nums are unique
            if nums[mid] < nums[hi] {
                // right is sorted
                hi = mid;
            } else {
                // now, nums[mid] > nums[hi], and nums[mid] >= nums[lo]
                lo = mid + 1;
            }
        }

        nums[lo]
    }
}

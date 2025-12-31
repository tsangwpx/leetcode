// Problem 41
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        use std::convert::TryFrom;
        let mut nums = nums;
        let limit = nums.len() as i32; // inclusive

        let mut idx = 0;

        // re-arrange values according to its value
        // put N to (N-1)th index if possible

        while idx < nums.len() {
            // for each position in nums
            let item = nums[idx];
            let pos = usize::try_from(item - 1).unwrap_or(nums.len());

            if pos < nums.len() && pos != idx && nums[pos] != item {
                nums.swap(idx, pos);
            } else {
                // otherwise, move on.
                idx += 1;
            }
        }

        // Now values within the index range are placed accordingly.
        for (idx, &value) in nums.iter().enumerate() {
            let expected = idx as i32 + 1;
            if value != expected {
                return expected;
            }
        }

        // the missing number is outside the inclusive limit
        limit + 1
    }
}

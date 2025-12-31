// Problem 2444
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut left = 0;
        let mut min_idx = usize::MAX;
        let mut max_idx = usize::MAX;
        let mut count = 0;

        for (idx, number) in nums.iter().copied().enumerate() {
            if number < min_k || number > max_k {
                // value out of bounds.
                // Start subarray next position.
                // reset min/max position.
                left = idx + 1;
                min_idx = usize::MAX;
                max_idx = usize::MAX;
                continue;
            }

            if number == min_k {
                min_idx = idx;
            }

            if number == max_k {
                max_idx = idx;
            }

            if min_idx == usize::MAX || max_idx == usize::MAX {
                // Missing min or max or both.
                continue;
            }

            // Now, we form subarrays ends at "idx" (inclusive)
            // because both min and max are found.
            // The least start index of subarrays is "left".
            // The greatest possible start index is min(min_idx, max_idx).
            // Then, we count the number of possible start indices
            // and add to the total count.

            // right is inclusive
            let right = min_idx.min(max_idx);
            count += right + 1 - left;
        }

        count as _
    }
}

// Problem 2210
impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        /*
         * 1. basic calculus? A second derivate need at least 3 points.
         * 2. bump the leftmost index in iterations
         * 3. make sure i < j < k and skip duplicated values
         * 4. check if (i, j, k) form a hill or valley
         * 5. exit if index out of range
         *
         * The following is equivalent to
         * 1. Remove deduplicated values in neighbors
         * 2. Check 3-tuple sequential fashion.
         */

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        let mut count = 0;

        while i < nums.len() {
            let iv = nums[i];
            while (i + 1) < nums.len() && nums[i + 1] == iv {
                i += 1;
            }

            j = j.max(i + 1);
            if j >= nums.len() {
                break;
            }
            let jv = nums[j];
            while (j + 1) < nums.len() && nums[j + 1] == jv {
                j += 1;
            }

            k = k.max(j + 1);
            if k >= nums.len() {
                break;
            }
            let kv = nums[k];
            while (k + 1) < nums.len() && nums[k + 1] == kv {
                k += 1;
            }

            if nums[j] > nums[i] && nums[j] > nums[k] {
                count += 1;
            } else if nums[j] < nums[i] && nums[j] < nums[k] {
                count += 1;
            }

            i += 1;
        }

        count
    }
}

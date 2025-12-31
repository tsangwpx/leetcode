// Problem 33
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        assert!(nums.len() >= 1);

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            let value = *nums.get(mid).unwrap();
            if value == target {
                return mid as i32;
            }

            let a = *nums.get(left).unwrap();
            let b = *nums.get(right).unwrap();

            if a <= value {
                // left half is sorted.
                if a <= target && target <= value {
                    // target in left half
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // right half is sorted
                if value <= target && target <= b {
                    // target in right half
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1i32
    }
}

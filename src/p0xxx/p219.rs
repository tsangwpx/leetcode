// Problem 219
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;

        if k == 0 {
            return false;
        }

        assert!(k >= 0);
        // let k = k as usize + 1;
        let window = k as usize + 1;

        let mut seen = HashSet::<i32>::new();

        for &number in nums.iter().take(window) {
            if !seen.insert(number) {
                return true;
            }
        }

        for (i, &number) in nums.iter().enumerate().skip(window) {
            seen.remove(&nums[i - window]);
            if !seen.insert(number) {
                return true;
            }
        }

        false
    }
}

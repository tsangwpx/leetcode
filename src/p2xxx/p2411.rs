// Problem 2411
impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut last_seen = vec![usize::MAX; 31];

        for (idx, item) in nums.iter().copied().enumerate().rev() {
            let mut idx2 = idx;
            for shift in 0..31 {
                if (item >> shift) & 1 == 1 {
                    last_seen[shift] = idx;
                } else if last_seen[shift] != usize::MAX {
                    idx2 = idx2.max(last_seen[shift]);
                }
            }

            res[idx] = (idx2 + 1 - idx) as i32;
        }

        res
    }
}

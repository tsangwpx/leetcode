// Problem 2200
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res = vec![];
        let mut j = 0;

        for i in 0..nums.len() {
            if nums[i] != key {
                continue;
            }

            j = j.max(i.saturating_sub(k));

            while j < nums.len() && j <= i + k {
                res.push(j as i32);
                j += 1;
            }
        }

        res
    }
}

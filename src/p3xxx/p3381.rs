// Problem 3381
impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut pfxmin = vec![i64::MAX; k as usize];
        *pfxmin.last_mut().unwrap() = 0;

        let mut sum = 0;
        let mut res = i64::MIN;

        for (idx, item) in nums.iter().copied().enumerate() {
            sum += item as i64;
            let idx = ((idx as i32) % k) as usize;

            if pfxmin[idx] != i64::MAX {
                res = res.max(sum - pfxmin[idx]);
            }

            pfxmin[idx] = pfxmin[idx].min(sum);
        }

        res
    }
}

// Problem 2302
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut count = 0;

        let mut sum = 0i64;
        let mut left = 0;

        for (idx, number) in nums.iter().copied().enumerate() {
            sum += i64::from(number);

            while left <= idx {
                let score = (idx + 1 - left) as i64 * sum;

                if score < k {
                    break;
                }

                sum -= i64::from(nums[left]);
                left += 1;
            }

            count += (idx + 1 - left) as i64;
        }

        count
    }
}

// Problem 2176
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for (i, a) in nums.iter().copied().enumerate() {
            for (j, b) in nums.iter().copied().enumerate().skip(i + 1) {
                if a == b && (i * j) as i32 % k == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

// Problem 2348
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut zlen = 0;

        for item in nums.iter().copied() {
            if item == 0 {
                zlen += 1;
                count += zlen;
            } else {
                zlen = 0;
            }
        }

        count
    }
}

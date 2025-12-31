// Problem 3487
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut seen = 0u128;
        let mut max_nonpositive = i32::MIN;

        for item in nums {
            if item > 0 && item < 128 {
                seen |= 1 << item;
            } else {
                max_nonpositive = max_nonpositive.max(item);
            }
        }

        let mut sum = 0;
        for shift in 1..128 {
            if (seen >> shift) & 1 == 1 {
                sum += shift;
            }
        }

        if sum > 0 { sum } else { max_nonpositive }
    }
}

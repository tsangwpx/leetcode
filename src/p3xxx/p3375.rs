// Problem 3375
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let bits = nums
            .into_iter()
            .fold(0u128, |acc, number| acc | (1u128 << number));

        let bad = (1u128 << k) - 1;
        if bits & bad != 0 {
            return -1;
        }

        (bits >> (k + 1)).count_ones() as _
    }
}

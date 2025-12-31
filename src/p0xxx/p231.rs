// Problem 231
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n >= 1 && n & (n - 1) == 0
    }
}

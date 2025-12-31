// Problem 1304
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut res = (1..n).collect::<Vec<_>>();

        // sum of AS
        res.push((n - 1) * (n) / -2);

        res
    }
}

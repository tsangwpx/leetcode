// Problem 2579
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = i64::from(n);
        let side = n * 2 - 1;
        let area = side * side;
        let short = n - 1;
        let triangle = short * (short + 1) / 2;
        let removed = triangle * 4;

        area - removed
    }
}

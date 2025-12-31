// Problem 2033
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut flatten = grid.into_iter().flatten().collect::<Vec<i32>>();
        flatten.sort_unstable();

        let median = flatten[flatten.len() / 2];
        let mut count = 0;

        for item in flatten.iter().copied() {
            let delta = (item - median).abs();
            let rem = delta % x;
            if rem != 0 {
                return -1;
            }
            count += delta / x;
        }

        count
    }
}

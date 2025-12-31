// Problem 1351
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let (count, _) = grid.iter().fold((0, n), |(acc, right), row| {
            let idx = row[0..right].partition_point(|&s| s >= 0);
            // println!("{} {}", row.len(), idx);

            (acc + row.len() - idx, right.min(idx + 1))
        });

        count as i32
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 2482
impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        assert!(grid.len() >= 1);

        let m = grid.len();
        let n = grid[0].len();
        for row in grid.iter() {
            assert!(row.len() == n);
        }

        let mut row_sums = vec![0i32; m];
        let mut col_sums = vec![0i32; n];

        for i in 0..m {
            row_sums[i] = grid[i].iter().sum();
        }

        for i in 0..n {
            col_sums[i] = (0..m).map(|s| grid[s][i]).sum();
        }

        let mut grid = grid;

        let c = (m + n) as i32;

        for i in 0..m {
            for j in 0..n {
                let ones = row_sums[i] + col_sums[j];
                grid[i][j] = 2 * ones - c;
            }
        }

        grid
    }
}

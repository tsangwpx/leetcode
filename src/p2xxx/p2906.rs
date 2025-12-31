// Problem 2906
impl Solution {
    pub fn construct_product_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MOD: i32 = 12345;

        let n = grid.len();
        let m = grid[0].len();

        let mut res = vec![vec![0; m]; n];
        let mut prod = 1;

        for (i, row) in grid[0..n].iter_mut().enumerate() {
            for (j, item) in row[0..m].iter_mut().enumerate() {
                *item = (*item) % MOD;
                prod = (prod * *item) % MOD;
                res[i][j] = prod;
            }
        }

        let mut prod = 1;

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                let prefix = if j == 0 {
                    if i == 0 { 1 } else { res[i - 1][m - 1] }
                } else {
                    res[i][j - 1]
                };
                let item = (prefix * prod) % MOD;
                res[i][j] = item;
                prod = (prod * grid[i][j]) % MOD;
            }
        }

        res
    }
}

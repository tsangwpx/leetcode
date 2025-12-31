// Problem 1277
impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        assert!(m >= 1 && n >= 1);

        let mut count = 0;
        let mut dp0 = vec![0; n];
        let mut dp1 = dp0.clone();

        for (i, row) in matrix.iter().enumerate() {
            assert!(row.len() == n);

            for (j, item) in row.iter().copied().enumerate() {
                if item == 0 {
                    dp1[j] = 0;
                } else if i == 0 || j == 0 {
                    dp1[j] = 1;
                } else {
                    dp1[j] = dp1[j - 1].min(dp0[j - 1]).min(dp0[j]) + 1;
                }
            }

            count += dp1.iter().sum::<i32>();

            std::mem::swap(&mut dp0, &mut dp1);
        }

        count
    }
}

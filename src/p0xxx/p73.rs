// Problem 73
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let zero_first_row = matrix[0].iter().any(|&s| s == 0);
        let zero_first_col = matrix.iter().any(|s| s[0] == 0);

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if zero_first_row {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        if zero_first_col {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

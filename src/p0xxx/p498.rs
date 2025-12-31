// Problem 498
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let lines = m + n - 1;

        let mut res = vec![];

        for k in 0..lines {
            let is_even = k % 2 == 0;

            if is_even {
                let mut i = k.min(m - 1);
                let mut j = k - i;

                while i < m && j < n {
                    res.push(mat[i][j]);
                    i = i.wrapping_sub(1);
                    j = j + 1;
                }
            } else {
                let mut j = k.min(n - 1);
                let mut i = k - j;

                while i < m && j < n {
                    res.push(mat[i][j]);
                    i = i + 1;
                    j = j.wrapping_sub(1);
                }
            }
        }

        res
    }
}

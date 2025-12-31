// Problem 1582
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        assert!(m >= 1 && m <= 100);
        assert!(n >= 1 && n <= 100);

        let mut hori_sums = vec![0u8; m];
        let mut vert_sums = vec![0u8; n];

        for (i, row) in mat.iter().enumerate() {
            assert!(row.len() == n);

            for (j, &item) in row.iter().enumerate() {
                hori_sums[i] += item as u8;
                vert_sums[j] += item as u8;
            }
        }

        // println!("{:?}", hori_sums);
        // println!("{:?}", vert_sums);

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if hori_sums[i] == 1 && vert_sums[j] == 1 && mat[i][j] == 1 {
                    res += 1;
                }
            }
        }

        res
    }
}

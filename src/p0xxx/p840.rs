// Problem 840
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        // brute-force
        // alternative, we could search for 5 in the middle for short-check,
        // and do the long-check if short-check passes.

        let m = grid.len();
        let n = grid[0].len();

        let mut count = 0;

        #[inline]
        fn is_magic_square(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
            if i + 2 >= grid.len() {
                return false;
            }

            let r1 = &grid[i];
            let r2 = &grid[i + 1];
            let r3 = &grid[i + 2];
            if j + 2 >= r1.len() || j + 2 >= r2.len() || j + 2 >= r3.len() {
                return false;
            }

            let mut sq = [0; 9];

            sq[0..3].copy_from_slice(&r1[j..j + 3]);
            sq[3..6].copy_from_slice(&r2[j..j + 3]);
            sq[6..9].copy_from_slice(&r3[j..j + 3]);

            let mut found = 0;

            for &item in sq.iter() {
                if item >= 1 && item <= 9 {
                    found |= 1 << item;
                }
            }

            if found != 0b1111111110 {
                return false;
            }

            let sum = sq[0] + sq[1] + sq[2];

            sq[3] + sq[4] + sq[5] == sum
                && sq[6] + sq[7] + sq[8] == sum
                && sq[0] + sq[3] + sq[6] == sum
                && sq[1] + sq[4] + sq[7] == sum
                && sq[2] + sq[5] + sq[8] == sum
                && sq[0] + sq[4] + sq[8] == sum
                && sq[2] + sq[4] + sq[6] == sum
        }

        for i in 0..m.saturating_sub(2) {
            for j in 0..n.saturating_sub(2) {
                if is_magic_square(&grid, i, j) {
                    count += 1;
                }
            }
        }

        count
    }
}

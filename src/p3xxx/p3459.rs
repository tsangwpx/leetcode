// Problem 3459
impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Direction {
            TL,
            TR,
            BR,
            BL,
        }

        impl Direction {
            #[inline]
            fn rotate(self) -> Self {
                match self {
                    Direction::TL => Direction::TR,
                    Direction::TR => Direction::BR,
                    Direction::BR => Direction::BL,
                    Direction::BL => Direction::TL,
                }
            }

            #[inline]
            fn index(self) -> usize {
                match self {
                    Direction::TL => 0,
                    Direction::TR => 1,
                    Direction::BR => 2,
                    Direction::BL => 3,
                }
            }

            #[inline]
            fn next(self, i: usize, j: usize) -> (usize, usize) {
                match self {
                    Direction::TL => (i.wrapping_sub(1), j.wrapping_sub(1)),
                    Direction::TR => (i.wrapping_sub(1), j.wrapping_add(1)),
                    Direction::BR => (i.wrapping_add(1), j.wrapping_add(1)),
                    Direction::BL => (i.wrapping_add(1), j.wrapping_sub(1)),
                }
            }
        }

        type CellInfo = [[i32; 2]; 4];

        let m = grid.len();
        let n = grid[0].len();

        fn dfs(
            dp: &mut Vec<Vec<CellInfo>>,
            grid: &Vec<Vec<i32>>,
            i: usize,
            j: usize,
            dir: Direction,
            turned: bool,
            expected: i32,
        ) -> i32 {
            if i >= grid.len() || j >= grid[i].len() {
                return 0;
            }

            if grid[i][j] != expected {
                // wrong step
                return 0;
            }

            // println!("dfs {} {} {:?} {} {}", i, j, dir, turned, expected);

            let dir_index = dir.index();
            let turn_index = if turned { 1usize } else { 0 };
            if dp[i][j][dir_index][turn_index] < 0 {
                dp[i][j][dir_index][turn_index] = 1;

                let mut steps = 0;

                let (h, k) = dir.next(i, j);
                steps = steps.max(dfs(dp, grid, h, k, dir, turned, 2 - expected));

                if !turned {
                    let dir = dir.rotate();
                    let (h, k) = dir.next(i, j);
                    steps = steps.max(dfs(dp, grid, h, k, dir, true, 2 - expected));
                }

                dp[i][j][dir_index][turn_index] = steps + 1;
            }

            dp[i][j][dir_index][turn_index]
        }

        let mut res = 0;

        let mut dp = vec![vec![[[-1; 2]; 4]; n]; m];
        for (i, row) in grid.iter().enumerate() {
            assert!(row.len() == n);
            for (j, item) in row.iter().copied().enumerate() {
                if item != 1 {
                    continue;
                }

                res = res.max(1);

                for dir in [Direction::TL, Direction::TR, Direction::BR, Direction::BL] {
                    let (h, k) = dir.next(i, j);

                    let steps = dfs(&mut dp, &grid, h, k, dir, false, 2);
                    if steps >= 1 {
                        // println!("{:?} {} {} {}", dir, i, j, steps + 1);
                        res = res.max(steps + 1);
                    }
                }
            }
        }

        res
    }
}

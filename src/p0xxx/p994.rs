// Problem 994
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for row in grid.iter() {
            assert!(row.len() == n);
        }

        const EMPTY: i32 = 0;
        const FRESH: i32 = 1;
        const ROTTEN: i32 = 2;

        #[derive(Debug)]
        struct FreshOrange {
            row: usize,
            col: usize,
            rotting: bool,
        }

        let mut fresh: Vec<FreshOrange> = vec![];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == FRESH {
                    fresh.push(FreshOrange {
                        row: i,
                        col: j,
                        rotting: false,
                    });
                }
            }
        }

        for minute in 0.. {
            let mut need_update = false;
            fresh.iter_mut().for_each(|orange| {
                let rotting = [
                    orange.row > 0 && grid[orange.row - 1][orange.col] == ROTTEN,
                    orange.row + 1 < m && grid[orange.row + 1][orange.col] == ROTTEN,
                    orange.col > 0 && grid[orange.row][orange.col - 1] == ROTTEN,
                    orange.col + 1 < n && grid[orange.row][orange.col + 1] == ROTTEN,
                ]
                .iter()
                .any(|s| *s);

                orange.rotting = rotting;
                need_update = need_update || rotting;
            });

            if !need_update {
                return if fresh.is_empty() { minute } else { -1 };
            }

            fresh.retain(|orange| {
                if orange.rotting {
                    grid[orange.row][orange.col] = ROTTEN
                }

                !orange.rotting
            });
        }

        unreachable!("the minute loop should have returned");
    }

    pub fn oranges_rotting2(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut fresh = vec![];
        let mut minutes = 0;

        const EMPTY: i32 = 0;
        const FRESH: i32 = 1;
        const ROTTEN: i32 = 2;

        loop {
            fresh.clear();

            let mut need_update = false;

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] != FRESH {
                        continue;
                    }

                    let rotting = [
                        i > 0 && grid[i - 1][j] == ROTTEN,
                        i + 1 < m && grid[i + 1][j] == ROTTEN,
                        j > 0 && grid[i][j - 1] == ROTTEN,
                        j + 1 < n && grid[i][j + 1] == ROTTEN,
                    ]
                    .iter()
                    .any(|s| *s);

                    fresh.push((i, j, rotting));
                    need_update |= rotting;
                }
            }

            if !need_update {
                return if fresh.is_empty() { minutes } else { -1 };
            }

            minutes += 1;

            for (i, j, rotting) in fresh.drain(..) {
                if rotting {
                    grid[i][j] = ROTTEN;
                }
            }
        }
    }
}

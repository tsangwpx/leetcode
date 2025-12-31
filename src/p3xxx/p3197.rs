// Problem 3197
impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        fn minimum_area(
            grid: &Vec<Vec<i32>>,
            istart: usize,
            istop: usize,
            jstart: usize,
            jstop: usize,
        ) -> Option<i32> {
            let mut imin = grid.len();
            let mut imax = 0;
            let mut jmin = grid[0].len();
            let mut jmax = 0;

            assert!(grid.len() >= istop);

            for i in istart..istop {
                let row = &grid[i];
                assert!(row.len() >= jstop);

                for j in jstart..jstop {
                    if row[j] == 1 {
                        imin = imin.min(i);
                        imax = imax.max(i);
                        jmin = jmin.min(j);
                        jmax = jmax.max(j);
                    }
                }
            }

            if imin <= imax && jmin <= jmax {
                Some((imax - imin + 1) as i32 * (jmax - jmin + 1) as i32)
            } else {
                None
            }
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut res = i32::MAX;

        // Layout
        //   1
        // -----
        // 2 | 3
        for imid in 1..m {
            let Some(a1) = minimum_area(&grid, 0, imid, 0, n) else {
                continue;
            };

            for jmin in 1..n {
                let Some(a2) = minimum_area(&grid, imid, m, 0, jmin) else {
                    continue;
                };

                let Some(a3) = minimum_area(&grid, imid, m, jmin, n) else {
                    continue;
                };

                res = res.min(a1 + a2 + a3);
            }
        }

        // Layout
        // 1 | 2
        // -----
        //   3
        for imid in 1..m {
            let Some(a3) = minimum_area(&grid, imid, m, 0, n) else {
                continue;
            };

            for jmid in 1..n {
                let Some(a1) = minimum_area(&grid, 0, imid, 0, jmid) else {
                    continue;
                };

                let Some(a2) = minimum_area(&grid, 0, imid, jmid, n) else {
                    continue;
                };

                res = res.min(a1 + a2 + a3);
            }
        }

        // Layout
        // 1 | 2
        //   | 3
        for jmid in 1..n {
            let Some(a1) = minimum_area(&grid, 0, m, 0, jmid) else {
                continue;
            };

            for imid in 1..m {
                let Some(a2) = minimum_area(&grid, 0, imid, jmid, n) else {
                    continue;
                };

                let Some(a3) = minimum_area(&grid, imid, m, jmid, n) else {
                    continue;
                };

                res = res.min(a1 + a2 + a3);
            }
        }

        // Layout
        // 1 | 3
        // 2 |
        for jmid in 1..n {
            let Some(a3) = minimum_area(&grid, 0, m, jmid, n) else {
                continue;
            };

            for imid in 1..m {
                let Some(a1) = minimum_area(&grid, 0, imid, 0, jmid) else {
                    continue;
                };

                let Some(a2) = minimum_area(&grid, imid, m, 0, jmid) else {
                    continue;
                };

                res = res.min(a1 + a2 + a3);
            }
        }

        // Layout
        // 1
        // 2
        // 3
        for i1 in 1..m {
            let Some(a1) = minimum_area(&grid, 0, i1, 0, n) else {
                continue;
            };

            for i2 in i1 + 1..m {
                let Some(a2) = minimum_area(&grid, i1, i2, 0, n) else {
                    continue;
                };

                let Some(a3) = minimum_area(&grid, i2, m, 0, n) else {
                    continue;
                };

                res = res.min(a1 + a2 + a3);
            }
        }

        // Layout
        // 1 | 2 | 3
        for j1 in 1..n {
            let Some(a1) = minimum_area(&grid, 0, m, 0, j1) else {
                continue;
            };

            for j2 in j1 + 1..n {
                let Some(a2) = minimum_area(&grid, 0, m, j1, j2) else {
                    continue;
                };

                let Some(a3) = minimum_area(&grid, 0, m, j2, n) else {
                    continue;
                };

                res = res.min(a1 + a2 + a3);
            }
        }

        res
    }
}

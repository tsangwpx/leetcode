// Problem 2267
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let m = grid.len();
        let n = grid[0].len();

        let mut dp0 = vec![HashSet::<i8>::new(); n];
        let mut dp1 = dp0.clone();

        dp0[0].insert(0);

        for row in grid.iter() {
            for (idx, item) in row.iter().copied().enumerate() {
                let delta = if item == '(' { 1 } else { -1 };
                for mut open in dp0[idx].drain() {
                    open += delta;
                    if open >= 0 && open < i8::MAX {
                        dp1[idx].insert(open);
                    }
                }

                if idx > 0 {
                    let (first, second) = dp1.split_at_mut(idx);
                    let prev = first.last_mut().unwrap();
                    let curr = second.first_mut().unwrap();

                    for mut open in prev.iter().copied() {
                        open += delta;
                        if open >= 0 && open < i8::MAX {
                            curr.insert(open);
                        }
                    }
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0.last().map(|s| s.contains(&0)).unwrap_or(false)
    }
}

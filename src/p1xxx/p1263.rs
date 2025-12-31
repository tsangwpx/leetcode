use std::ops::{Add, Sub};

// Problem 1263
impl Solution {
    pub fn min_push_box(mut grid: Vec<Vec<char>>) -> i32 {
        use std::collections::VecDeque;

        let m = grid.len();
        let n = grid[0].len();

        let mut box_pos = None;
        let mut player_pos = None;
        let mut target_pos = None;

        for i in 0..m {
            assert!(grid[i].len() == n);
            for j in 0..n {
                match grid[i][j] {
                    'S' => {
                        player_pos = Some((i, j));
                        grid[i][j] = '.';
                    }
                    'B' => {
                        box_pos = Some((i, j));
                        grid[i][j] = '.';
                    }
                    'T' => {
                        target_pos = Some((i, j));
                        grid[i][j] = '.';
                    }
                    _ => {}
                }
            }
        }

        let target_pos = target_pos.unwrap();

        let mut player_visited = vec![false; n * m];
        let mut pending = VecDeque::new();

        let mut push_done = vec![[false; 4]; n * m];

        let mut queue = VecDeque::new();
        queue.push_back((0, box_pos.unwrap(), player_pos.unwrap()));

        while let Some((pushes, box_pos, player_pos)) = queue.pop_front() {
            if box_pos == target_pos {
                return pushes;
            }

            let (box_row, box_col) = box_pos;
            let (player_row, player_col) = player_pos;

            // initialize the player reachable map
            player_visited.fill(false);
            // mark the box position is visited so that player cannot walk though the box
            player_visited[box_row * n + box_col] = true;
            player_visited[player_row * n + player_col] = true;

            pending.clear();
            pending.push_back((player_row, player_col));

            let pushes = pushes + 1;
            let pairs = [
                (
                    (box_row.wrapping_sub(1), box_col), // push up
                    (box_row.wrapping_add(1), box_col),
                    0usize,
                ),
                (
                    (box_row.wrapping_add(1), box_col), // push down
                    (box_row.wrapping_sub(1), box_col),
                    1,
                ),
                (
                    (box_row, box_col.wrapping_sub(1)), // left
                    (box_row, box_col.wrapping_add(1)),
                    2,
                ),
                (
                    (box_row, box_col.wrapping_add(1)), // right
                    (box_row, box_col.wrapping_sub(1)),
                    3,
                ),
            ];

            for ((new_row, new_col), (push_row, push_col), direction) in pairs {
                if new_row >= m || new_col >= n || push_row >= m || push_col >= n {
                    // out of bounds
                    continue;
                }
                if grid[new_row][new_col] != '.' || grid[push_row][push_col] != '.' {
                    // box cannot be moved there or player cannot go there
                    continue;
                }
                if push_done[new_row * n + new_col][direction] {
                    continue;
                }

                let mut reachable = player_visited[push_row * n + push_col];

                if !reachable {
                    // continue the bfs if unreachable
                    while let Some((from_row, from_col)) = pending.pop_front() {
                        let steps = [
                            (from_row.wrapping_sub(1), from_col),
                            (from_row.wrapping_add(1), from_col),
                            (from_row, from_col.wrapping_sub(1)),
                            (from_row, from_col.wrapping_add(1)),
                        ];

                        for (next_row, next_col) in steps {
                            if next_row >= m || next_col >= n {
                                // out of bounds
                                continue;
                            }
                            if player_visited[next_row * n + next_col] {
                                // skip visited
                                continue;
                            }
                            // if (next_row, next_col) == (box_row, box_col) {
                            //     continue;
                            // }
                            if grid[next_row][next_col] != '.' {
                                // cannot go there
                                continue;
                            }

                            player_visited[next_row * n + next_col] = true;
                            pending.push_back((next_row, next_col));
                        }

                        if player_visited[push_row * n + push_col] {
                            reachable = true;
                            break;
                        }
                    }
                }

                if reachable {
                    // Player can reach the push position
                    queue.push_back((pushes, (new_row, new_col), (box_row, box_col)));
                    push_done[new_row * n + new_col][direction] = true;
                }
            }
        }

        -1
    }
}

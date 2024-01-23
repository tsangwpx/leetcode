mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1926
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        let mut maze = maze;
        let m = maze.len();
        let n = maze[0].len();

        let mut queue = VecDeque::with_capacity(m.max(n));
        let entrance_cell = (entrance[0] as usize, entrance[1] as usize);
        queue.push_back((0, entrance_cell.0, entrance_cell.1));

        while let Some((dist, row, col)) = queue.pop_front() {
            // println!("{}: {} {}", dist, row, col);
            maze[row][col] = 'x';

            let next_cells = [
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ];

            for (next_row, next_col) in next_cells {
                if next_row >= m || next_col >= n {
                    continue;
                }

                if maze[next_row][next_col] != '.' {
                    continue;
                }

                // print!("{} {:?}: {:?}", dist, (row, col), (next_row, next_col));

                if next_row == 0 || next_row + 1 == m || next_col == 0 || next_col + 1 == n {
                    // println!("{}", queue.len());
                    return dist + 1;
                }

                maze[next_row][next_col] = 'x';
                queue.push_back((dist + 1, next_row, next_col));
            }
        }

        -1
    }

    pub fn nearest_exit2(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let mut nodes = HashMap::new();
        let mut exits = vec![];

        for (i, row) in maze.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '+' {
                    continue;
                }
                let idx = nodes.len();
                let idx = nodes.entry((i, j)).or_insert(idx).clone();

                if i == 0 || i + 1 == maze.len() || j == 0 || j + 1 == row.len() {
                    exits.push(idx);
                }
            }
        }

        let entrance_id = nodes
            .get(&(entrance[0] as usize, entrance[1] as usize))
            .copied()
            .unwrap();

        let mut edges = vec![vec![]; nodes.len()];

        for (i, row) in maze.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '+' {
                    continue;
                }

                let node_id = nodes.get(&(i, j)).unwrap().clone();

                // only inspect bottom and right cells because of reflection
                if i + 1 < maze.len() && maze[i + 1][j] == '.' {
                    let bottom_id = nodes.get(&(i + 1, j)).unwrap().clone();
                    edges[node_id].push(bottom_id);
                    edges[bottom_id].push(node_id);
                }

                if j + 1 < row.len() && row[j + 1] == '.' {
                    let right_id = nodes.get(&(i, j + 1)).unwrap().clone();
                    edges[node_id].push(right_id);
                    edges[right_id].push(node_id);
                }
            }
        }
        // println!("{:?}", nodes);
        // println!("{:?}", edges);
        // println!("{:?}", exits);
        // println!("{:?}", entrance_id);
        drop(nodes);

        type Distance = i16;

        let mut dists = vec![Distance::MAX; edges.len()];
        dists[entrance_id] = 0;

        let mut heap = BinaryHeap::new();
        heap.extend(
            dists
                .iter()
                .copied()
                .enumerate()
                .map(|(idx, d)| (Reverse(d), idx)),
        );

        while let Some((Reverse(dist), idx)) = heap.pop() {
            if dists[idx] == Distance::MAX || dists[idx] != dist {
                continue;
            }

            let new_dist = dist + 1;

            for neigbour_idx in edges[idx].iter().copied() {
                let neighbor_dist = dists[neigbour_idx];
                if new_dist < neighbor_dist {
                    dists[neigbour_idx] = new_dist;
                    heap.push((Reverse(new_dist), neigbour_idx));
                }
            }
        }

        // println!("{:?}", dists);

        exits
            .iter()
            .copied()
            .filter_map(|exit_id| {
                // skip extrance node and unreachable nodes
                if exit_id == entrance_id || dists[exit_id] == Distance::MAX {
                    None
                } else {
                    Some(dists[exit_id])
                }
            })
            .min()
            .unwrap_or(-1) as i32
    }
}

mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2065
impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut neighbours = vec![[(u16::MAX, u8::MAX); 4]; values.len()];

        for triplet in edges.iter() {
            let time = triplet[2] as u8;
            let v = triplet[1] as u16;
            let u = triplet[0] as u16;

            neighbours[v as usize]
                .iter_mut()
                .filter(|(s, _)| *s == u16::MAX)
                .next()
                .map(|entry| *entry = (u, time));

            neighbours[u as usize]
                .iter_mut()
                .filter(|(s, _)| *s == u16::MAX)
                .next()
                .map(|entry| *entry = (v, time));
        }

        const INDEX_NODE: usize = 4;
        let mut visited = vec![[false; INDEX_NODE + 1]; values.len()];

        fn dfs(
            values: &Vec<i32>,
            neighbours: &Vec<[(u16, u8); 4]>,
            best_quality: &mut i32,
            visited: &mut Vec<[bool; 5]>,
            max_time: i32,
            mut quality: i32,
            node: u16,
        ) {
            let node = node as usize;

            // is the first visit to this node? Take the value if yes.
            let value_taken = visited[node][INDEX_NODE];

            if !value_taken {
                visited[node][INDEX_NODE] = true;
                quality += values[node];
            }

            if node == 0 {
                // update the best value when visiting the origin.
                *best_quality = quality.max(*best_quality);
            }

            // println!(
            //     "Visit {}: {} {} {} {}",
            //     node, quality, max_time, taken, *best_quality
            // );

            for (edge_idx, &(dst, time)) in neighbours[node].iter().enumerate() {
                if dst as usize >= values.len() {
                    break;
                }
                let time = time as i32;
                if time > max_time {
                    // cant reach that node due to time
                    continue;
                }

                // dont visit again from the same direction
                let edge_taken = visited[node][edge_idx];

                if !edge_taken {
                    visited[node][edge_idx] = true;

                    dfs(
                        values,
                        neighbours,
                        best_quality,
                        visited,
                        max_time - time,
                        quality,
                        dst,
                    );
                    // undo
                    visited[node][edge_idx] = false;
                }
            }

            if !value_taken {
                // undo
                visited[node][INDEX_NODE] = false;
            }
        }

        let mut best_quality = values[0];

        dfs(
            &values,
            &neighbours,
            &mut best_quality,
            &mut visited,
            max_time,
            0,
            0,
        );
        best_quality
    }
}

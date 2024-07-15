mod leetcode_prelude;

use std::vec;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 797
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut visited = vec![false; graph.len()];
        let mut backtrace = vec![vec![]; graph.len()];
        let mut pending = vec![0];

        while let Some(src) = pending.pop() {
            for dst in graph[src].iter().copied() {
                let dst = dst as usize;
                backtrace[dst].push(src as i32);

                if dst >= visited.len() || visited[dst] {
                    continue;
                }
                visited[dst] = true;
                pending.push(dst);
            }
        }

        let mut res = vec![];
        let mut pending = vec![vec![graph.len() as i32 - 1]];

        while let Some(path) = pending.pop() {
            let start = path.last().copied().unwrap();
            for src in backtrace[start as usize].iter().copied() {
                let mut path2 = path.clone();
                path2.push(src);
                if src == 0 {
                    path2.reverse();
                    res.push(path2);
                } else {
                    pending.push(path2);
                }
            }
        }

        res
    }
}

mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 403
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut visited = vec![HashSet::<i32>::new(); stones.len()];

        fn dfs(stones: &[i32], visited: &mut [HashSet<i32>], last_step: i32) -> bool {
            if stones.len() <= 1 {
                return true;
            }

            for step in 1.max(last_step - 1)..=(last_step + 1) {
                let next_stone = stones[0] + step;
                // println!("{}: {} {}", stones[0], step, next_stone);
                match stones.binary_search(&next_stone) {
                    Err(_) => {}
                    Ok(idx) => {
                        if !visited[idx].insert(step) {
                            continue;
                        }
                        if dfs(&stones[idx..], &mut visited[idx..], step) {
                            return true;
                        }
                    }
                }
            }

            false
        }

        dfs(&stones, &mut visited, 0)
    }

    pub fn can_cross2(stones: Vec<i32>) -> bool {
        use std::collections::HashMap;
        use std::collections::HashSet;

        assert!(stones.len() >= 2);

        let dest = stones.last().copied().unwrap();
        if dest == 1 {
            return true;
        }

        let mut dp = stones
            .iter()
            .copied()
            .map(|s| (s, HashSet::<i32>::new()))
            .collect::<HashMap<_, _>>();

        if let Some(steps) = dp.get_mut(&1) {
            steps.insert(1);
        } else {
            return false;
        }

        for idx in 1..stones.len() {
            let stone = stones[idx];
            let arrival_steps = std::mem::take(dp.get_mut(&stone).unwrap());
            for last_step in arrival_steps {
                for step in [last_step - 1, last_step, last_step + 1] {
                    if step <= 0 {
                        continue;
                    }

                    let next_stone = stone + step;
                    if next_stone == dest {
                        return true;
                    }
                    if let Some(next_steps) = dp.get_mut(&next_stone) {
                        next_steps.insert(step);
                    }
                }
            }
        }

        false
    }
}

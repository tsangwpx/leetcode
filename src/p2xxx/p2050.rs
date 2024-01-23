mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2050
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut npredecessors = vec![0; n];
        let mut successors = vec![vec![]; n];

        for link in relations.iter() {
            let next = link[1] as usize - 1;
            let prev = link[0] as usize - 1;
            assert!(next < n && prev < n);

            npredecessors[next] += 1;
            successors[prev].push(next);
        }

        let mut completed_time = vec![0; n];
        let mut ready = npredecessors
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, count)| if count == 0 { Some(i) } else { None })
            .collect::<Vec<_>>();

        ready
            .iter()
            .copied()
            .for_each(|s| completed_time[s] = time[s]);

        while let Some(idx) = ready.pop() {
            let finish_time = completed_time[idx];

            successors[idx].iter().copied().for_each(|s| {
                completed_time[s] = completed_time[s].max(finish_time + time[s]);

                npredecessors[s] -= 1;
                if npredecessors[s] == 0 {
                    ready.push(s);
                }
            });
        }

        completed_time.iter().copied().max().unwrap()
    }
}

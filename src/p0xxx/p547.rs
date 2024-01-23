mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 547
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        let mut visited = vec![false; is_connected.len()];
        let mut queue = vec![];

        for city in 0..visited.len() {
            if visited[city] {
                // this city is used
                continue;
            }

            // Start a new group of cities!
            count += 1;
            visited[city] = true;

            // push to the checking queue
            queue.push(city);

            while let Some(i) = queue.pop() {
                for j in 0..visited.len() {
                    if visited[j] {
                        // skip if used
                        continue;
                    }

                    if is_connected[i][j] == 1 {
                        // mark this as used and push it in queue
                        visited[j] = true;
                        queue.push(j);
                    }
                }
            }
        }

        unsafe {
        }

        count
    }
}

mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1496
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        use std::collections::HashSet;

        let mut visited = HashSet::<(i16, i16)>::with_capacity(path.len());

        let mut x = 0i16;
        let mut y = 0i16;

        visited.insert((x, y));

        for dir in path.bytes() {
            match dir {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                b'W' => x -= 1,
                _ => {}
            }

            // println!("{} {}", x, y);

            if !visited.insert((x, y)) {
                return true;
            }
        }

        false
    }
}

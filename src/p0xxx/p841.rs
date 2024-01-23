mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 841
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut locked = vec![true; rooms.len()];
        locked[0] = false;

        let mut locked_count = rooms.len() - 1;
        let mut queue = vec![0];

        while let Some(room_idx) = queue.pop() {
            for &key in rooms[room_idx as usize].iter() {
                if locked[key as usize] {
                    locked[key as usize] = false;
                    queue.push(key);
                    locked_count -= 1;
                }
            }
        }

        locked_count == 0
    }
}

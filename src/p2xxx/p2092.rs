mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 2092
impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut meetings = meetings;
        meetings.sort_unstable_by_key(|s| s[2]);
        let meetings = meetings;

        let mut known = vec![false; n as usize];
        known[0] = true;
        known[first_person as usize] = true;

        let mut new_friends = vec![];
        let mut table = HashMap::<i32, Vec<i32>>::new();

        let mut offset = 0;
        while offset < meetings.len() {
            let time = meetings[offset][2];
            let count = meetings
                .iter()
                .skip(offset)
                .filter(|s| s[2] == time)
                .count();

            table.clear();

            for meeting in meetings[offset..offset + count].iter() {
                let b = meeting[1];
                let a = meeting[0];

                match (known[a as usize], known[b as usize]) {
                    (true, true) => {}
                    (true, false) => {
                        known[b as usize] = true;
                        new_friends.push(b);
                    }
                    (false, true) => {
                        known[a as usize] = true;
                        new_friends.push(a);
                    }
                    (false, false) => {
                        table.entry(a).or_default().push(b);
                        table.entry(b).or_default().push(a);
                    }
                }
            }

            offset += count;

            while let Some(person) = new_friends.pop() {
                if let Some(friends) = table.remove(&person) {
                    for friend in friends {
                        if !known[friend as usize] {
                            known[friend as usize] = true;
                            new_friends.push(friend);
                        }
                    }
                }
            }
        }

        known
            .into_iter()
            .enumerate()
            .filter_map(|(idx, secret)| if secret { Some(idx as i32) } else { None })
            .collect::<Vec<_>>()
    }
}

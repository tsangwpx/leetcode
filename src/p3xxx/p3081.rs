mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 3081
impl Solution {
    pub fn minimize_string_value(s: String) -> String {
        let mut counter = [0i32; 26];
        let mut indices = [0u8; 26];

        for idx in 0..indices.len() {
            indices[idx] = idx as u8;
        }

        let mut unknown = 0;

        for ch in s.bytes() {
            match ch {
                b'?' => unknown += 1,
                b'a'..=b'z' => {
                    let idx = (ch - b'a') as usize;
                    if idx < 26 {
                        counter[idx] += 1;
                    }
                }
                _ => {}
            }
        }

        if unknown == 0 {
            return s;
        }

        indices.sort_by_key(|&s| counter[s as usize]);

        let mut width = 1;
        let mut height = counter[indices[0] as usize];
        let mut remaining = unknown;

        for idx in 1..indices.len() {
            let idx = indices[idx] as usize;
            let delta = counter[idx] - height;
            let consumed = delta * width;

            if consumed > remaining {
                let q = remaining / width;
                let consumed = q * width;
                height += q;
                remaining -= consumed;
                break;
            }

            width += 1;
            height += delta;
            remaining -= consumed;
        }

        if remaining >= width {
            let q = remaining / width;
            let consumed = q * width;
            height += q;
            remaining -= consumed;
        }

        // println!("{} {} {}: {:?}", height, remaining, unknown, counter);

        let mut unused = [0i32; 26];

        for idx in 0..26 {
            if height >= counter[idx] {
                unused[idx] = height - counter[idx];
                if remaining > 0 {
                    unused[idx] += 1;
                    remaining -= 1;
                }
            }
        }

        // println!("unused: {:?}", unused);

        let mut idx = 0;
        let mut s = s.into_bytes();
        for ch in s.iter_mut() {
            if *ch != b'?' {
                continue;
            }

            while idx < unused.len() && unused[idx] == 0 {
                idx += 1;
            }

            if idx < unused.len() {
                unused[idx] -= 1;
                *ch = idx as u8 + b'a';
            }
        }

        unsafe { String::from_utf8_unchecked(s) }
    }

    pub fn minimize_string_value2(s: String) -> String {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut counter = [0; 26];
        let mut count = 0;
        for ch in s.bytes() {
            match ch {
                b'?' => count += 1,
                b'a'..=b'z' => {
                    let idx = (ch - b'a') as usize;
                    if idx < 26 {
                        counter[idx] += 1;
                    }
                }
                _ => {}
            }
        }

        let mut pq = counter
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, count)| Reverse((count, idx as u8)))
            .collect::<BinaryHeap<_>>();

        counter.fill(0);
        for _ in 0..count {
            let Some(mut peek) = pq.peek_mut() else {
                unreachable!();
            };

            let Reverse((count, idx)) = &mut *peek;
            *count += 1;
            counter[*idx as usize] += 1;
        }

        let mut idx = 0;

        let mut s = s.into_bytes();
        for ch in s.iter_mut() {
            if *ch != b'?' {
                continue;
            }

            while idx < counter.len() && counter[idx] == 0 {
                idx += 1;
            }

            if idx < counter.len() {
                counter[idx] -= 1;
                *ch = idx as u8 + b'a';
            }
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}

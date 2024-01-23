mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 68
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let whitespace = " ".repeat(max_width);
        let mut i = 0;
        let mut res = vec![];

        while i < words.len() {
            let mut width = words[i].len();
            let mut j = i + 1;

            while j < words.len() && width + 1 + words[j].len() <= max_width {
                width += 1 + words[j].len();
                j += 1;
            }

            let word_count = j - i;
            let left_justified = word_count == 1 || j == words.len();

            if left_justified {
                let mut line = String::with_capacity(max_width);
                line.push_str(&words[i]);
                i += 1;

                while i < j {
                    line.push(' ');
                    line.push_str(&words[i]);
                    i += 1;
                }

                line.push_str(&whitespace[line.len()..]);
                res.push(line);
            } else {
                let mut line = String::with_capacity(max_width);
                let space_count = max_width - width + word_count - 1;
                let space_slots = word_count - 1;
                let (sep, mut extra) = (space_count / space_slots, space_count % space_slots);
                println!("{} {}: {} {}", space_count, space_slots, sep, extra);

                line.push_str(&words[i]);
                i += 1;

                while i < j {
                    if extra > 0 {
                        line.push_str(&whitespace[0..sep + 1]);
                        extra -= 1;
                    } else {
                        line.push_str(&whitespace[0..sep]);
                    }
                    line.push_str(&words[i]);
                    i += 1;
                }
                res.push(line);
            }
        }

        res
    }
}

// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 32
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<u16> = Vec::with_capacity(100);
        let mut longest = 0u16;
        let mut dp = vec![0u16; s.len()];

        for (idx, ch) in s.bytes().into_iter().enumerate() {
            match ch {
                b'(' => stack.push(idx as u16),
                b')' => {
                    match stack.pop() {
                        None => {}
                        Some(pos) => {
                            let prev_length = pos
                                .checked_sub(1) // linebreak
                                .map_or(0, |prev_idx| dp[prev_idx as usize]);

                            let length = idx as u16 + 1 - pos;
                            let total_length = length + prev_length;
                            longest = longest.max(total_length);
                            dp[idx] = total_length;
                        }
                    };
                }
                _ => unreachable!(),
            }
        }

        longest as i32
    }
}

mod B {
    struct Solution {}

    // Problem 32
    impl Solution {
        pub fn longest_valid_parentheses(s: String) -> i32 {
            use std::collections::BinaryHeap;

            let mut stack: Vec<usize> = Vec::with_capacity(100);
            let mut heap = BinaryHeap::<(u16, usize, u16)>::with_capacity(64);
            let mut longest = 0i32;

            for (idx, ch) in s.bytes().into_iter().enumerate() {
                match ch {
                    b'(' => stack.push(idx),
                    b')' => {
                        match stack.pop() {
                            None => {}
                            Some(pos) => {
                                let level = stack.len() as u16 + 1;
                                let mut prev_length = 0;

                                while let Some(&(prev_level, prev_target, prev_size)) = heap.peek()
                                {
                                    if prev_target == pos {
                                        // let's do a check rather than prove some conditons.
                                        prev_length = prev_size;
                                    }

                                    if prev_level >= level {
                                        heap.pop();
                                    } else {
                                        break;
                                    }
                                }

                                let length = (idx + 1 - pos) as u16;
                                let total_length = length + prev_length;
                                longest = longest.max(total_length as i32);
                                heap.push((level, idx + 1, total_length));
                            }
                        };
                    }
                    _ => unreachable!(),
                }
            }

            longest
        }
    }
}

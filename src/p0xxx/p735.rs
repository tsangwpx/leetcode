mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 735
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering;

        let mut stack = vec![0i32; 0];

        'outer: for &asteroid in &asteroids {
            // If stack is empty, top is leftward, or asteroid rightward,
            // add to stack directly
            if asteroid > 0 || stack.is_empty() || *stack.last().unwrap() < 0 {
                stack.push(asteroid);
                continue;
            }

            // asteroid is -ve

            'inner: while let Some(&top) = stack.last() {
                if top < 0 {
                    // both moving leftward
                    stack.push(asteroid);
                    continue 'outer;
                }

                // now top is +ve
                match top.abs().cmp(&asteroid.abs()) {
                    Ordering::Equal => {
                        // both explode!
                        stack.pop();
                        continue 'outer;
                    }
                    Ordering::Greater => {
                        // asteroid explode
                        continue 'outer;
                    }
                    Ordering::Less => {
                        // top explode
                        stack.pop();
                        continue 'inner;
                    }
                }

                // here is unreachable!
            }

            // stack is empty or all moving left
            stack.push(asteroid);
        }

        stack
    }
}

mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 150
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(16);

        for token in tokens {
            match token.as_str() {
                op @ ("+" | "-" | "*" | "/") => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let c = match op {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => unreachable!(),
                    };

                    stack.push(c);
                }
                number => stack.push(str::parse::<i32>(number).unwrap()),
            }
        }

        stack.pop().unwrap()
    }
}

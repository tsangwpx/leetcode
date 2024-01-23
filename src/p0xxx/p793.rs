use std::ops::ControlFlow;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 793
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        assert!(temperatures.len() >= 1);

        let mut stack = vec![0usize];
        let mut res = vec![0i32; temperatures.len()];

        for (now, &value) in temperatures.iter().enumerate().skip(1) {
            while let Some(&past) = stack.last() {
                if temperatures[past] < value {
                    // This is the first day warmer than the past day.
                    res[past] = (now - past) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(now);
        }

        res
    }
}

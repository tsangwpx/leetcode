// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 70
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        assert!(n >= 1 && n <= 45);
        let mut steps = [0; 46];

        // initialize steps
        steps[0] = 1;
        steps[1] = 1;
        steps[2] = 2;

        for i in 3..=n as usize {
            steps[i] = steps[i - 1] + steps[i - 2];
        }

        steps[n as usize]
    }
}

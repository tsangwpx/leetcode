// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    // Problem 22
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        assert!(n >= 1);

        let n = n as u8;

        let mut res = vec![];

        Self::solve(&mut res, n, n, "".to_owned());

        // res.sort();
        res
    }

    fn solve(res: &mut Vec<String>, open: u8, close: u8, mut tmp: String) {
        match (open, close) {
            (0, 0) => res.push(tmp),
            (_, _) => {
                if close > open {
                    Self::solve(res, open, close - 1, tmp.clone() + ")")
                }

                if open > 0 {
                    Self::solve(res, open - 1, close, tmp + "(");
                }
            }
        }
    }
}

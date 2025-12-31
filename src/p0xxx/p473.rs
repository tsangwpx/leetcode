use std::collections::HashMap;

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        use std::collections::HashMap;

        let total = matchsticks
            .iter()
            .fold(0i64, |accum, &num| accum + i64::from(num));
        if matchsticks.len() < 4 || total < 4 || total % 4 != 0 {
            return false;
        }
        let perimeter = total / 4;
        let mut memo = HashMap::<u32, bool>::new();

        /// Choose sticks to form a valid side (by combination) and repeat 4 times
        fn recurse(
            matchsticks: &Vec<i32>,
            perimeter: i64,
            memo: &mut HashMap<u32, bool>,
            visited: u32,
            done: u32,
            start: usize,
            side: i64,
        ) -> bool {
            if done == 4 {
                return true;
            }

            if side == perimeter {
                // restart from the beginning
                return recurse(matchsticks, perimeter, memo, visited, done + 1, 0, 0);
            }

            let key = visited;
            if let Some(&res) = memo.get(&key) {
                return res;
            }

            let mut res = false;

            for i in start..matchsticks.len() {
                let flag = 1 << i;
                if visited & flag == flag {
                    continue;
                }

                let side_new = side + i64::from(matchsticks[i]);
                if side_new > perimeter {
                    continue;
                }

                if recurse(
                    matchsticks,
                    perimeter,
                    memo,
                    visited | flag,
                    done,
                    i + 1,
                    side_new,
                ) {
                    res = true;
                    break;
                }
            }

            memo.insert(key, res);
            res
        }

        recurse(&matchsticks, perimeter, &mut memo, 0, 0, 0, 0)
    }
}

fn main() {
    println!("Hello World");
}

#![feature(bench_black_box)]

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        use std::cmp::max;

        let m = m as usize;
        let n = n as usize;
        assert!(m <= 200);
        assert!(n <= 200);

        let mut dp = [0i64; 256 * 256];

        macro_rules! loc {
            ($h:expr, $w:expr) => {
                {($h) * 256 + ($w)}
            };
        }

        macro_rules! dp {
            ($h:expr, $w:expr) => {
                *unsafe { dp.get_unchecked_mut(loc!($h, $w)) }
            };
        }


        for entry in prices {
            if entry.len() < 3 {
                unsafe { std::hint::unreachable_unchecked(); }
            }
            dp!(entry[0] as usize, entry[1] as usize) = entry[2] as i64;
        }

        for height in 1..=m {
            for width in 1..=n {
                let mut best = dp!(height, width);
                // horizontal cut
                for mid in 1..=(height / 2) {
                    best = max(best, dp!(mid, width) + dp!(height - mid, width));
                }

                // vertical cut
                for mid in 1..=(width / 2) {
                    best = max(best, dp!(height, mid) + dp!(height, width - mid));
                }

                // dp[loc!(height, width)] = best;
                dp!(height, width) = best;
            }
        }

        dp!(m, n)
    }
}

struct Solution {}

fn main() {
    use std::hint::black_box;

    let m = black_box(100);
    let n = black_box(100);
    let prices = black_box(vec![vec![1, 2, 3], vec![4, 5, 6]]);

    println!("{}", Solution::selling_wood(m, n, prices));
}
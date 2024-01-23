mod leetcode_prelude;

use std::ffi::FromVecWithNulError;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1531
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let k = k as usize;

        // dp[i][j] is the minimum length when encoding s[0..i] with at most j characters deleted
        // dp size is (s.len() + 1)-by-(k + 1)
        let mut dp: Vec<Vec<i32>> = Vec::with_capacity(s.len() + 1);

        dp.push(vec![0; k + 1]); // zero size input is always zero
        dp.resize(s.len() + 1, vec![i32::MAX; k + 1]);

        assert!(s.len() < dp.len());

        for i in 1..=s.len() {
            for j in 0..=k {
                // encoding s[0..i] with at most j characters deleted
                // s[i - 1] is the last character in the substring

                // case 1: delete s[i - 1]
                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                }

                // case 2: keep s[i - 1]
                // try to encode s[..i + 1] backward with at most j characters deleted

                // a better name to represent the end of substring
                let stop = i;
                let mut start = i - 1;
                let mut deleted = 0; // how many deletions done

                // encode it alone, ie. encode(s[..start]) + s[start]
                dp[i][j] = dp[i][j].min(dp[start][j - deleted] + 1);

                let ch = s.bytes().nth(start).unwrap();

                while start > 0 {
                    start -= 1;

                    if s.bytes().nth(start).unwrap() != ch {
                        if deleted >= j {
                            // no deletions are available. Cannot encode s[start..i]
                            break;
                        }

                        deleted += 1;
                    }

                    // how many time ch is repeated along some characters deleted
                    let count = stop - start - deleted;

                    // encoded length of s[start..stop] with some characters deleted
                    let encoded_len = match count {
                        1 => 1,
                        2..=9 => 2,
                        10..=99 => 3,
                        100..=999 => 4,
                        _ => unreachable!(),
                    };
                    // so the total length depends on how many deletions left.
                    dp[i][j] = dp[i][j].min(dp[start][j - deleted] + encoded_len);
                }
            }
        }

        dp[s.len()][k]
    }
}

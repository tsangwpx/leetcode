mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 649
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        #[derive(Debug, Clone, Copy)]
        enum Senate {
            Radiant,
            Dire,
        }

        use std::collections::VecDeque;

        let mut this_round = senate
            .into_bytes()
            .into_iter()
            .map(|ch| match ch {
                b'R' => Senate::Radiant,
                b'D' => Senate::Dire,
                _ => unreachable!(),
            })
            .collect::<VecDeque<Senate>>();

        let mut next_round = VecDeque::<Senate>::new();
        let mut radiant_banned = 0;
        let mut dire_banned = 0;

        loop {
            let mut radient_count = 0;
            let mut dire_count = 0;

            while let Some(senate) = this_round.pop_front() {
                match senate {
                    Senate::Radiant => {
                        if radiant_banned > 0 {
                            radiant_banned -= 1;
                        } else {
                            dire_banned += 1;
                            radient_count += 1;
                            next_round.push_back(senate);
                        }
                    }
                    Senate::Dire => {
                        if dire_banned > 0 {
                            dire_banned -= 1;
                        } else {
                            radiant_banned += 1;
                            dire_count += 1;
                            next_round.push_back(senate);
                        }
                    }
                }
            }

            if radient_count == 0 {
                return "Dire".to_owned();
            } else if dire_count == 0 {
                return "Radiant".to_owned();
            }

            std::mem::swap(&mut this_round, &mut next_round);
        }
    }
}

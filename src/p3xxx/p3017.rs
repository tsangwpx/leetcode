mod leetcode_prelude;

use std::iter::FromIterator;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 3017
impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let n = n as usize;
        let (x, y) = if x <= y { (x, y) } else { (y, x) };
        let (x, y) = (x as usize, y as usize);
        assert!(x <= n && y <= n && x <= y);

        let mut res = vec![0; n];

        const MODE_ADD: i64 = 2;
        const MODE_ADD_TWICE: i64 = 4;
        const MODE_REMOVE: i64 = -2;
        const MODE_REMOVE_TWICE: i64 = -4;

        #[inline(always)]
        fn linear(counter: &mut Vec<i64>, size: usize, mode: i64) {
            // println!("linear: {} {}", size, mode);
            // linear is only meaningful if it contains 2 nodes
            if size >= 2 {
                for i in 0..size.saturating_sub(1) {
                    counter[i] += mode * (size - i - 1) as i64;
                }
            }
        }

        // number of houses in cycle
        let cycle = y - x + 1;

        // use cycle as shortcut, e.g. from 1 to n
        let major_linear = if cycle >= 2 { n - cycle + 2 } else { n };
        linear(&mut res, major_linear, MODE_ADD);
        // println!("major: {:?}", res);

        let (cycle_stop, even) = if (cycle & 1) == 1 {
            // ood, one middle nodes
            (cycle / 2, false)
        } else {
            // even, two middle nodes
            (cycle / 2 - 1, true)
        };

        if cycle >= 3 {
            // cycle is only meaningful if it contains at least 3 nodes
            res[0] += 2 * (cycle as i64);

            for i in 1..cycle_stop {
                res[i] += 2 * (cycle as i64);
            }

            if even {
                res[cycle_stop] += cycle as i64;
            }

            // we may double counting the x and y in the major linear case
            linear(&mut res, 2, MODE_REMOVE);

            // println!("cycle={}: {:?}", cycle, res);

            #[inline(always)]
            fn cycle_external(counter: &mut Vec<i64>, external: usize, cycle: usize) {
                if external == 0 || cycle <= 2 {
                    return;
                }
                let half = cycle / 2;
                let even = (cycle & 1) == 0;

                if even {
                    // add upper, 1..x..x+mid
                    linear(counter, external + half, MODE_ADD);

                    // remove duplicate 1..x
                    linear(counter, external + 1, MODE_REMOVE);

                    // remove duplicate x..x + mid
                    linear(counter, half, MODE_REMOVE);

                    // add lower, 1..x..y..x+mid
                    linear(counter, external + half + 1, MODE_ADD);

                    // remove duplicate 1..x y
                    linear(counter, external + 2, MODE_REMOVE);

                    // remove x..y..x+mid
                    linear(counter, half + 1, MODE_REMOVE);

                    linear(counter, 2, MODE_ADD);
                } else {
                    // upper and lower are same length
                    linear(counter, external + half + 1, MODE_ADD_TWICE);

                    // remove duplicate 1..x
                    linear(counter, external + 1, MODE_REMOVE);

                    // remove duplicate 1..x y
                    linear(counter, external + 2, MODE_REMOVE);

                    // remove duplicate x..x + mid and x..x y (x+mid-1) but we did double delete here
                    linear(counter, half + 1, MODE_REMOVE_TWICE);

                    // add back x..y
                    linear(counter, 2, MODE_ADD);
                }
            }

            // println!("left");
            cycle_external(&mut res, x - 1, cycle);
            // println!("right");
            cycle_external(&mut res, n - y, cycle)
        }

        res
    }
}

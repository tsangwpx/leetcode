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

// Problem 3013
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BTreeSet;
        use std::collections::BinaryHeap;

        let k = k as usize;
        let dist = dist as usize;

        assert!(nums.len() >= k && k >= 3 && dist >= 1);

        // since nums[0] must be the first subarray, the window is k - 1 in size
        // set up the sliding window of k - 1 items
        let winlen = k - 2;
        let mut window = BTreeSet::new();
        let mut window_cost = 0;

        for i in 1..k - 1 {
            let item = nums[i];
            window.insert((item, i));
            window_cost += i64::from(item);
        }

        let mut min_cost = window_cost + i64::from(nums[k - 1]);
        let mut queue = BinaryHeap::with_capacity(dist);

        // println!("cost={}, win={:?}, queue={:?}", min_cost, window, queue);

        for idx in (k - 1)..nums.len() {
            let item = nums[idx];
            // idx is the beginning of the last subarray, which is not in the window

            // 1. Discard nums[idx - dist - 1] from window if any, and update cost
            // we dont remove nums[0] even though it is not in the window.
            // start from dist + 2, which is nums[1] = nums[(dist + 2) - dist - 1]
            if idx >= dist + 2 {
                let removed_idx = idx - dist - 1;
                let removed_item = nums[removed_idx];

                if window.remove(&(removed_item, removed_idx)) {
                    window_cost -= i64::from(removed_item);
                }
            }

            loop {
                // 2. Popping the smallest item from queue, discard if out of range
                let (small_item, small_idx) = match queue.peek().copied() {
                    Some((Reverse(small_item), small_idx)) => (small_item, small_idx),
                    None => break,
                };
                if !(idx <= small_idx + dist) {
                    queue.pop();
                    continue;
                }

                if window.len() < winlen {
                    queue.pop();

                    // Fill the window if not full
                    window.insert((small_item, small_idx));
                    window_cost += i64::from(small_item);

                    if window.len() < winlen {
                        continue;
                    } else {
                        // we dont need to try swapping the large and the small
                        break;
                    }
                }

                // 3. Peek the largest item from window
                let (large_item, large_idx) = window.last().copied().unwrap();

                if large_item <= small_item {
                    // large item is not large
                    break;
                }

                // 4. Swap them if largest is indeed larger than smallest
                window.pop_last();
                window.insert((small_item, small_idx));

                let mut peek = queue.peek_mut().unwrap();
                *peek = (Reverse(large_item), large_idx);

                window_cost -= i64::from(large_item);
                window_cost += i64::from(small_item);
            }

            // 5. Add this item to queue
            queue.push((Reverse(item), idx));

            // 6. update min_cost
            min_cost = min_cost.min(window_cost + i64::from(item));

            // if window.len() <= 10 {
            //     println!(
            //         "idx={}, cost={}, window={:?}, heap={:?}",
            //         idx,
            //         window_cost + i64::from(item),
            //         window,
            //         queue
            //     );
            // }
        }

        i64::from(nums[0]) + min_cost
    }
}

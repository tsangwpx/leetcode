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
        use std::collections::btree_map::Entry as BEntry;
        use std::collections::hash_map::Entry;
        use std::collections::BTreeMap;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let k = k as usize;
        let dist = dist as usize;

        assert!(nums.len() >= k && k >= 3 && dist >= 1);

        // since nums[0] must be the first subarray, the window is k - 1 in size
        // set up the sliding window of k - 1 items
        let winlen_max = k - 2;
        let mut winlen = 0;
        let mut window = BTreeMap::new();
        let mut window_cost = 0;

        for i in 1..k - 1 {
            let item = nums[i];
            window
                .entry(item)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            winlen += 1;
            window_cost += i64::from(item);
        }

        let mut min_cost = window_cost + i64::from(nums[k - 1]);
        let mut queue = BinaryHeap::with_capacity(dist);
        let mut missing = HashMap::<i32, i32>::new();

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

                match window.entry(removed_item) {
                    BEntry::Occupied(mut entry) => {
                        let count = entry.get_mut();
                        if *count <= 1 {
                            entry.remove_entry();
                        } else {
                            *count -= 1;
                        }
                        winlen -= 1;
                        window_cost -= i64::from(removed_item);
                    }
                    BEntry::Vacant(_) => {
                        missing
                            .entry(removed_item)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }

            loop {
                // 2. Popping the smallest item from queue, discard if out of range
                let small_item = match queue.peek().copied() {
                    Some(Reverse(small_item)) => small_item,
                    None => break,
                };

                match missing.entry(small_item) {
                    Entry::Vacant(_) => {}
                    Entry::Occupied(mut entry) => {
                        queue.pop();

                        let count = entry.get_mut();
                        if *count <= 1 {
                            entry.remove_entry();
                        } else {
                            *count -= 1;
                        }

                        continue;
                    }
                }

                if winlen < winlen_max {
                    queue.pop();

                    // Fill the window if not full
                    window
                        .entry(small_item)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    winlen += 1;
                    window_cost += i64::from(small_item);

                    if winlen < winlen_max {
                        continue;
                    } else {
                        // we dont need to try swapping the large and the small
                        break;
                    }
                }

                // 3. Peek the largest item from window
                let mut large_entry = window.last_entry().unwrap();
                let large_item = *large_entry.key();

                if large_item <= small_item {
                    // large item is not large
                    break;
                }

                // 4. Swap them if largest is indeed larger than smallest
                let large_count = large_entry.get_mut();
                if *large_count <= 1 {
                    large_entry.remove();
                } else {
                    *large_count -= 1;
                    drop(large_entry);
                }

                window
                    .entry(small_item)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                let mut peek = queue.peek_mut().unwrap();
                *peek = Reverse(large_item);

                window_cost -= i64::from(large_item);
                window_cost += i64::from(small_item);
            }

            // 5. Add this item to queue
            queue.push(Reverse(item));

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

// Problem 2637
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        let k = i64::from(k);
        let mut result = 0;

        // sliding window

        // starting index
        let mut left = 0;

        // pair inside the window
        let mut pair_count = 0;

        // item count inside the window
        let mut counter = HashMap::<i32, i32>::new();

        for idx in 0..nums.len() {
            {
                let item = nums[idx];
                let item_count = *counter.entry(item).and_modify(|s| *s += 1).or_insert(1) as i64;

                pair_count += item_count - 1;

                // println!(
                //     "{}: item={}, count={}, pair_count={}",
                //     idx, item, item_count, pair_count
                // );
            }

            while pair_count >= k && left < idx {
                // assert!(idx > left, "{}: {}, {}", idx, left, pair_count);
                let item = nums[left];
                let item_count = counter.get(&item).copied().unwrap() as i64;
                let new_pair_count = pair_count - (item_count - 1) as i64;

                if new_pair_count < k {
                    break;
                }

                left += 1;

                if item_count == 1 {
                    counter.remove(&item);
                } else {
                    *counter.get_mut(&item).unwrap() -= 1;
                }

                pair_count = new_pair_count;
            }

            if pair_count >= k {
                result += left as i64 + 1;
            }

            // println!(
            //     "{}: left={}  pair_count={}; result={}",
            //     idx, left, pair_count, result
            // );
        }

        result
    }
}

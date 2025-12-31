// Problem 1751
impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        // https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/solutions/1052442/simple-dp-solution-recursion-memoization-c/?envType=daily-question&envId=2025-07-08
        // very slow because it can be speed up using binary search

        let mut events = events
            .into_iter()
            .map(|s| {
                let &[start, end, value] = s.as_slice() else {
                    panic!();
                };

                (start, end, value)
            })
            .collect::<Vec<(i32, i32, i32)>>();

        events.sort_unstable_by_key(|s| s.0);

        let events = events;
        let k = k as usize;

        let mut memo = vec![vec![-1; k + 1]; events.len() + 1];

        #[inline]
        fn solve(
            memo: &mut Vec<Vec<i32>>,
            events: &Vec<(i32, i32, i32)>,
            idx: usize,
            k: usize,
        ) -> i32 {
            if idx >= events.len() || k == 0 {
                return 0;
            }

            if memo[idx][k] >= 0 {
                return memo[idx][k];
            }

            let mut idx2 = idx + 1;
            while idx2 < events.len() {
                if events[idx2].0 > events[idx].1 {
                    break;
                }
                idx2 += 1;
            }

            memo[idx][k] = std::cmp::max(
                solve(memo, events, idx + 1, k),
                events[idx].2 + solve(memo, events, idx2, k - 1),
            );
            memo[idx][k]
        }

        solve(&mut memo, &events, 0, k)
    }
}

// Problem 1865
use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    counter2: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let counter2 = nums2.iter().copied().fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_default() += 1;
            map
        });

        Self {
            nums1,
            nums2,
            counter2,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let old_value = self.nums2[index];
        let new_value = old_value + val;
        self.nums2[index] = new_value;

        {
            let old_count = self.counter2.entry(old_value).or_default();
            *old_count -= 1;
            if *old_count == 0 {
                self.counter2.remove(&old_value);
            }
        }

        *self.counter2.entry(new_value).or_default() += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut count = 0;

        for item in self.nums1.iter().copied() {
            let other = tot - item;
            count += self.counter2.get(&other).copied().unwrap_or(0);
        }

        count
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
fn _f() {}

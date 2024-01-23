impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        #[inline(always)]
        fn make_tuple(prefix: &Vec<i32>, a: i32, b: i32) -> Vec<i32> {
            // Ok. this is Vec indeed.
            let mut vec = Vec::with_capacity(prefix.len() + 1);
            vec.extend_from_slice(prefix.as_slice());
            vec.push(a);
            vec.push(b);
            vec
        }

        #[inline(always)]
        fn recurse(nums: &[i32], k: usize, target: i64, prefix: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
            assert!(k >= 2);
            assert!(k <= nums.len());

            if k >= 3 {
                // For k = 3
                // Idx: 0, 1, 2, 3, 4, 5
                // Seq: A, B, C, D, E, F
                //      ^first   ^last iteration

                for i in 0..(nums.len() - k + 1) {
                    let n = nums[i];
                    if i > 0 && n == nums[i - 1] {
                        // Idx: 0, 1, 2, 3
                        // Seq: B, B, B, B
                        //         ^  ^  ^ skipped
                        //      ^emit every num once in prefix
                        continue;
                    }

                    let target2 = target - n as i64;
                    prefix.push(n);
                    recurse(&nums[i + 1..], k - 1, target2, prefix, results);
                    prefix.pop();
                }
            } else {
                let mut left = 0;
                let mut right = nums.len() - 1;

                while left < right {
                    // Find numbers which sum to target from left and right
                    let sum = nums[left] as i64 + nums[right] as i64;
                    let cmp = sum.cmp(&target);

                    if cmp.is_eq() {
                        results.push(make_tuple(prefix, nums[left], nums[right]));
                    }

                    if cmp.is_le() {
                        left += 1;

                        while left < right && nums[left] == nums[left - 1] {
                            // Skip repeated num from right
                            left += 1;
                        }
                    }

                    if cmp.is_ge() {
                        right -= 1;

                        while left < right && nums[right] == nums[right + 1] {
                            // Skip repeated num from right
                            right -= 1;
                        }
                    }
                }
            }
        }

        let mut results = Vec::new();
        if nums.len() >= 4 {
            nums.sort_unstable();
            let mut prefix = Vec::with_capacity(4);
            recurse(&nums[..], 4, target as i64, &mut prefix, &mut results);
        }
        results
    }


    pub fn four_sum2(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        use std::collections::{BTreeMap, HashMap, HashSet};
        use std::ops::Bound;

        nums.sort_unstable();
        assert!(nums.len() < 256);

        type PairSet = HashSet<(i32, i32)>;
        let mut table = HashMap::<i64, BTreeMap<u8, PairSet>>::new();
        let mut result = HashSet::<(i32, i32, i32, i32)>::with_capacity(nums.len());

        for i in 0..nums.len() {
            let x = nums[i];

            for j in (i + 1)..nums.len() {
                let y = nums[j];

                let sum = nums[i] as i64 + nums[j] as i64;
                let difference = target as i64 - sum;

                if let Some(peers) = table.get(&difference) {
                    for (_, other) in peers.range((Bound::Unbounded, Bound::Excluded(i as u8))) {
                        for &(p, q) in other.iter() {
                            result.insert((p, q, x, y));
                        }
                    }
                }

                table.entry(sum)
                    .or_insert_with(|| BTreeMap::new())
                    .entry(j as u8)
                    .or_insert_with(|| PairSet::new())
                    .insert((x, y));
            }
        }

        // println!("{:?}", result);

        result.into_iter()
            .map(|(a, b, c, d)| vec![a, b, c, d])
            .collect::<Vec<Vec<i32>>>()
    }
}

struct Solution {}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

fn main() {
    let wf = Solution::four_sum(vec![1, 2, 3, 4, 5, 6, 7], 10);
    // wf.f("a".to_string(), "e".to_string());
}
// Problem 18
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        nums.sort_unstable();

        let mut res = Vec::new();

        fn recurse(
            nums: &[i32],
            target: i32,
            res: &mut Vec<Vec<i32>>,
            start: usize,
            ksum: usize,
            prefix: &mut Vec<i32>,
        ) {
            // println!("{}: target={} prefix={:?}", ksum, target, prefix);
            if ksum >= 3 {
                let mut idx = start;
                while idx < nums.len() {
                    let num = nums[idx];

                    if let Some(target2) = target.checked_sub(num) {
                        prefix.push(num);

                        recurse(nums, target2, res, idx + 1, ksum - 1, prefix);

                        prefix.pop();
                    };

                    idx += 1;

                    while idx < nums.len() && nums[idx] == num {
                        idx += 1;
                    }
                }
            } else {
                let mut idx1 = start;
                let mut idx2 = nums.len() - 1;

                while idx1 < idx2 {
                    let num1 = nums[idx1];
                    let num2 = nums[idx2];
                    let delta = target as i64 - num1 as i64 - num2 as i64;

                    if delta == 0 {
                        let mut row = Vec::with_capacity(prefix.len() + 2);
                        row.clone_from(prefix);
                        row.push(num1);
                        row.push(num2);
                        res.push(row);
                    }

                    if delta >= 0 {
                        idx1 += 1;

                        while idx1 < idx2 && nums[idx1] == num1 {
                            idx1 += 1;
                        }
                    }

                    if delta <= 0 {
                        idx2 -= 1;

                        while idx1 < idx2 && nums[idx2] == num2 {
                            idx2 -= 1;
                        }
                    }
                }
            }
        }

        recurse(&nums[..], target, &mut res, 0, 4, &mut vec![]);

        res
    }

    pub fn four_sum3(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        fn recurse(
            nums: &[i32],
            k: usize,
            target: i64,
            prefix: &mut Vec<i32>,
            results: &mut Vec<Vec<i32>>,
        ) {
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

                table
                    .entry(sum)
                    .or_insert_with(|| BTreeMap::new())
                    .entry(j as u8)
                    .or_insert_with(|| PairSet::new())
                    .insert((x, y));
            }
        }

        // println!("{:?}", result);

        result
            .into_iter()
            .map(|(a, b, c, d)| vec![a, b, c, d])
            .collect::<Vec<Vec<i32>>>()
    }
}

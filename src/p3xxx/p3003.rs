// Problem 3003
impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        // idea from
        // https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations/solutions/4520860/dynamic-programming-and-bit-manipulation/
        use std::collections::HashMap;
        use std::hash::Hash;

        let mut dp0 = HashMap::with_capacity(26 * 2);
        let mut dp1 = dp0.clone();

        #[inline]
        fn update_max<K>(dp: &mut HashMap<K, i32>, key: K, count: i32)
        where
            K: Eq + Hash,
        {
            dp.entry(key)
                .and_modify(|part_count| *part_count = count.max(*part_count))
                .or_insert(count);
        }

        // initialize the map
        let first = s.bytes().nth(0).unwrap();
        for ch in b'a'..=b'z' {
            let bit = 1 << (ch - b'a');

            update_max(&mut dp0, (bit, ch == first), 0);
        }

        for ch in s.bytes().skip(1) {
            let bit = 1 << (ch - b'a');

            for ((prev, can_change), count) in dp0.drain() {
                let bit_count = i32::count_ones(prev) as i32;

                if prev & bit == 0 {
                    if bit_count == k {
                        update_max(&mut dp1, (bit, can_change), count + 1);
                    } else {
                        update_max(&mut dp1, (prev | bit, can_change), count);
                    }
                    continue;
                } else if can_change {
                    // this bit may be changed.
                    // to change or not to change is a question.
                    update_max(&mut dp1, (prev, can_change), count);

                    for shift in 0..26 {
                        let bit = 1 << shift;

                        if prev & bit == bit {
                            // this bit is seen before
                            continue;
                        }

                        if bit_count == k {
                            // place the changed bit in a new partitioin
                            update_max(&mut dp1, (bit, false), count + 1);
                        } else {
                            update_max(&mut dp1, (prev | bit, false), count);
                        }
                    }
                } else {
                    update_max(&mut dp1, (prev, can_change), count);
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        // println!("{:?}", dp0);

        dp0.drain().map(|(_, count)| count).max().unwrap_or(0) + 1
    }
}

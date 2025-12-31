// Problem 3043
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        #[inline(always)]
        fn len(n: i32) -> i32 {
            match n {
                0 => 0,
                1..=9 => 1,
                10..=99 => 2,
                100..=999 => 3,
                1000..=9999 => 4,
                10000..=99999 => 5,
                100000..=999999 => 6,
                1000000..=9999999 => 7,
                10000000..=99999999 => 8,
                100000000..=999999999 => 9,
                _ => 10,
            }
        }

        let mut prefixes = HashSet::with_capacity(arr1.len());
        for mut num in arr1.iter().copied() {
            prefixes.insert(num);

            while num >= 10 {
                num /= 10;
                prefixes.insert(num);
            }
        }

        // maximum is the largest prefix so far.
        let mut maximum = 0;

        for mut num in arr2.iter().copied() {
            // the solution can be improved if the number is larger than maximum
            while num > maximum {
                if prefixes.contains(&num) {
                    maximum = maximum.max(num);
                    break;
                }
                num /= 10;
            }
        }

        len(maximum)
    }

    pub fn longest_common_prefix2(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        #[inline(always)]
        fn len(n: i32) -> i32 {
            match n {
                0 => 0,
                1..=9 => 1,
                10..=99 => 2,
                100..=999 => 3,
                1000..=9999 => 4,
                10000..=99999 => 5,
                100000..=999999 => 6,
                1000000..=9999999 => 7,
                10000000..=99999999 => 8,
                100000000..=999999999 => 9,
                _ => 10,
            }
        }

        fn compute_prefixes(arr: &[i32]) -> HashSet<i32> {
            let mut prefixes = HashSet::with_capacity(arr.len());
            for mut num in arr.iter().copied() {
                prefixes.insert(num);

                while num >= 10 {
                    num /= 10;
                    prefixes.insert(num);
                }
            }

            prefixes
        }

        let prefixes1 = compute_prefixes(&arr1);
        let prefixes2 = compute_prefixes(&arr2);

        len(prefixes1
            .intersection(&prefixes2)
            .max()
            .copied()
            .unwrap_or(0))
    }
}

// Problem 454
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        use std::collections::HashMap;

        fn count_numbers(nums: &[i32]) -> HashMap<i32, usize> {
            let mut counter = HashMap::new();
            for number in nums.iter().copied() {
                counter
                    .entry(number)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            counter
        }

        fn reduce(left: &HashMap<i32, usize>, right: &HashMap<i32, usize>) -> HashMap<i32, usize> {
            let mut res = HashMap::new();
            for (&n1, &f1) in left.iter() {
                for (&n2, &f2) in right.iter() {
                    let sum = n1 + n2;
                    let total = f1 * f2;
                    res.entry(sum)
                        .and_modify(|count| *count += total)
                        .or_insert(total);
                }
            }
            res
        }

        let table1 = reduce(&count_numbers(&nums1), &count_numbers(&nums2));
        let table2 = reduce(&count_numbers(&nums3), &count_numbers(&nums4));

        let mut res = 0;
        for (&item, &c1) in table1.iter() {
            if let Some(&c2) = table2.get(&-item) {
                res += c1 * c2;
            }
        }

        res as i32
    }
}

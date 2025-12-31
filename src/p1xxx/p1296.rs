// Problem 1296
impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        let mut counter = HashMap::<i32, i32>::new();

        for number in nums.iter().copied() {
            *counter.entry(number).or_default() += 1;
        }

        for number in nums.iter().copied() {
            if let Some(count) = counter.get(&number).copied() {
                if count == 0 {
                    continue;
                }
            }

            let mut start = number;

            while let Some(count) = counter.get(&(start - 1)).copied() {
                if count >= 1 {
                    start -= 1;
                } else {
                    break;
                }
            }

            while let Some(decrement) = counter.get(&start).copied() {
                if decrement >= 1 {
                    for target in start..start + k {
                        if let Some(count2) = counter.get_mut(&target) {
                            if *count2 < decrement {
                                return false;
                            }
                            *count2 -= decrement;
                        } else {
                            return false;
                        }
                    }
                }

                start += 1;
            }
        }

        true
    }
}

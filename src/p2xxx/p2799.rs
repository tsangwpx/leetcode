// Problem 2799
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let size = nums.iter().copied().collect::<HashSet<i32>>().len();

        let mut counter = HashMap::<i32, i32>::new();
        let mut left = 0;
        let mut count = 0;

        for (idx, number) in nums.iter().copied().enumerate() {
            *counter.entry(number).or_default() += 1;

            if counter.len() >= size {
                while left < idx {
                    let prev = nums[left];
                    let prev_count = counter.get_mut(&prev).unwrap();

                    if *prev_count == 1 {
                        break;
                    }

                    *prev_count -= 1;
                    left += 1;
                }

                count += left as i32 + 1;
            }
        }

        count
    }
}

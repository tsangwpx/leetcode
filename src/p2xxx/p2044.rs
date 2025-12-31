// Problem 2044
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let target = nums.iter().copied().fold(0, |acc, number| acc | number);

        let mut count = 0;

        for combination in 1usize..(1 << nums.len()) {
            let mut value = 0;

            for (idx, number) in nums.iter().copied().enumerate() {
                if combination & (1 << idx) != 0 {
                    value |= number;
                }
            }

            if target == value {
                count += 1;
            }
        }

        count
    }
}

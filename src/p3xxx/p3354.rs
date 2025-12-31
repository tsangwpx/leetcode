// Problem 3354
impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();

        let mut count = 0;
        let mut left = 0;

        for item in nums.iter().copied() {
            left += item;

            if item == 0 {
                let right = sum - left;

                if left == right {
                    // both directions
                    count += 2;
                } else if (left - right).abs() == 1 {
                    // either direction but not both
                    count += 1;
                }
            }
        }

        count
    }
}

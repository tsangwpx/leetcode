// Problem 719
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums.last().copied().unwrap() - nums.first().copied().unwrap();

        fn count_pairs(nums: &[i32], dist: i32) -> i32 {
            let mut count = 0;
            let mut limit = 0;

            for start in 0..nums.len() {
                limit = limit.max(start);
                while limit + 1 < nums.len() && nums[limit + 1] - nums[start] <= dist {
                    limit += 1;
                }

                count += limit - start;
            }

            count as i32
        }

        while left < right {
            let dist = left + (right - left) / 2;

            // count number of pairs whose distance <= "dist"
            let count = count_pairs(&nums, dist);

            if k > count {
                left = dist + 1;
            } else {
                right = dist;
            }
        }

        left
    }
}

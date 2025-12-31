// Problem 611
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut count = 0;

        for k in 2..nums.len() {
            // k is the longest side
            // i is the shortest side
            // j is in between
            let mut i = 0;
            let mut j = k - 1;

            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    count += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }

        count as i32
    }
}

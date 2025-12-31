// Problem 2419
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut longest = 1;

        let mut prev = 0;
        let mut max = 0;
        let mut len = 1;

        for item in nums.iter().copied() {
            match item.cmp(&max) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    if item == prev {
                        len += 1;
                        longest = longest.max(len);
                    } else {
                        len = 1;
                    }
                }
                std::cmp::Ordering::Greater => {
                    max = item;
                    len = 1;
                    longest = 1;
                }
            }

            prev = item;
        }

        longest
    }
}

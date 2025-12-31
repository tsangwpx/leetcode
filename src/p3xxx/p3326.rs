// Problem 3326
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;

        'outer: for idx in (0..nums.len().saturating_sub(1)).rev() {
            let number = nums[idx];
            if number <= nums[idx + 1] {
                continue;
            }

            'inner: for divisor in 2..number {
                if divisor * divisor > number {
                    break 'inner;
                }

                let (q, r) = (number / divisor, number % divisor);
                if r == 0 {
                    // println!("{} {} {}", idx, number, q);
                    nums[idx] = divisor;
                    count += 1;
                    break;
                }
            }

            if nums[idx] > nums[idx + 1] {
                return -1;
            }
        }

        count
    }
}

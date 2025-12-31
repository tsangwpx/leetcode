// Problem 643
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        use std::collections::VecDeque;

        let mut deque = VecDeque::with_capacity(k as usize);
        let mut sum = 0i32;

        for i in 0..k as usize {
            deque.push_back(nums[i]);
            sum += nums[i];
        }

        let mut maximum = sum as f64 / k as f64;

        for i in k as usize..nums.len() {
            sum -= deque.pop_front().unwrap();
            sum += nums[i];
            deque.push_back(nums[i]);
            maximum = maximum.max(sum as f64 / k as f64);
        }

        maximum
    }
}

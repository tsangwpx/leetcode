use core::num;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 239
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;

        const WINDOW_LIMIT: usize = 0;
        let k = k as usize;

        assert!(k >= 1);
        assert!(nums.len() >= k);

        if k == 1 {
            return nums;
        } else if k <= WINDOW_LIMIT {
            return nums
                .windows(k)
                .into_iter()
                .map(|s| *s.iter().max().unwrap())
                .collect();
        } else if k == nums.len() {
            return vec![nums.into_iter().max().unwrap()];
        }

        // deque store index whose values are decreasing from front to back
        let mut deque = VecDeque::<usize>::with_capacity(k);
        let mut res = Vec::with_capacity(nums.len() - k + 1);

        for (idx, &number) in nums.iter().enumerate() {
            if let Some(&front_idx) = deque.front() {
                // Remove front if out of window
                if front_idx + k <= idx {
                    deque.pop_front();
                }
            }

            // Popping smaller or equal items from back
            while let Some(&back_idx) = deque.back() {
                if number < nums[back_idx] {
                    break;
                }

                deque.pop_back();
            }

            // push this to deque
            // If its empty, this is the max
            // Otherwise, this is the min so far
            deque.push_back(idx);

            if idx + 1 >= k {
                res.push(nums[*deque.front().unwrap()]);
            }
        }

        res
    }

    pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;

        const WINDOW_LIMIT: usize = 4;
        let k = k as usize;

        assert!(k >= 1);
        assert!(nums.len() >= k);

        if k == 1 {
            return nums;
        } else if k <= WINDOW_LIMIT {
            return nums
                .windows(WINDOW_LIMIT)
                .into_iter()
                .map(|s| *s.iter().max().unwrap())
                .collect();
        } else if k == nums.len() {
            return vec![nums.into_iter().max().unwrap()];
        }

        let mut removed = BinaryHeap::<i32>::with_capacity(k);
        let mut window = BinaryHeap::with_capacity(k);
        window.extend(nums.iter().take(k - 1).map(|&s| s));

        let mut res = Vec::with_capacity(nums.len() - k + 1);

        for (idx, &number) in nums.iter().enumerate().skip(k - 1) {
            if idx >= k {
                removed.push(nums[idx - k]);
            }

            while !removed.is_empty() && removed.peek() == window.peek() {
                removed.pop();
                window.pop();
            }

            window.push(number);
            res.push(*window.peek().unwrap());
        }

        res
    }
}

// Problem 3542
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        // increasing stack
        const MAX_NUM: i32 = 10i32.pow(5);
        let mut stack = Vec::with_capacity(nums.len().min(MAX_NUM as usize));
        let mut count = 0;

        for item in nums.iter().copied() {
            while !stack.is_empty() && stack.last().copied().unwrap() > item {
                stack.pop();
                count += 1;
            }

            if stack.last().copied() != Some(item) {
                stack.push(item);
            }
        }

        count += stack.len() as i32;
        if stack.first().copied() == Some(0) {
            // dont operate on zeros
            count -= 1;
        }

        count
    }
}

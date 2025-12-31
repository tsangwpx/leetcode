// Problem 3355
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut changes = vec![0; nums.len() + 1];

        for item in queries.iter() {
            let &[left, right] = item.as_slice() else {
                unreachable!();
            };
            changes[left as usize] += 1;
            changes[right as usize + 1] -= 1;
        }

        let mut decrement = 0;

        for i in 0..nums.len() {
            decrement += changes[i];
            if nums[i] > decrement {
                return false;
            }
        }

        true
    }
}

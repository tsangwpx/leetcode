// Problem 55
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut reachable = 0usize; // inclusive

        for (idx, &step) in nums.iter().enumerate() {
            if idx > reachable {
                break;
            }

            reachable = reachable.max(idx + step as usize);
        }

        reachable + 1 >= nums.len()
    }
}

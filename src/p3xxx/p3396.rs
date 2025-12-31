// Problem 3396
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        const LIMIT: usize = 100 + 1;
        let mut seen = [false; LIMIT];

        for (idx, number) in nums.iter().copied().enumerate().rev() {
            let number = number as usize;
            if number >= LIMIT {
                unreachable!();
            }

            if seen[number] {
                return ((idx + 3) / 3) as i32;
            }

            seen[number] = true;
        }

        0
    }
}

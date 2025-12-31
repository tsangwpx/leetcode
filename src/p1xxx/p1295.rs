// Problem 1295
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for number in nums.into_iter() {
            match number {
                10..=99 | 1000..=9999 | 100000..=999999 => count += 1,
                _ => {}
            }
        }

        count
    }
}

// Problem 3289
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .copied()
            .fold([0u8; 101], |mut counter, item| {
                let idx = item as usize;

                if idx < counter.len() {
                    counter[idx] += 1;
                }

                counter
            })
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(idx, count)| if count >= 2 { Some(idx as i32) } else { None })
            .collect::<Vec<i32>>()
    }
}

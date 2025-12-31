// Problem 137
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        nums.into_iter()
            .fold(HashMap::<i32, u8>::new(), |mut table, number| {
                table
                    .entry(number)
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);

                table
            })
            .into_iter()
            .filter_map(|(k, v)| if v == 1 { Some(k) } else { None })
            .next()
            .unwrap()
    }
}

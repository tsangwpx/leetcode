fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 560
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut total = 0;
        let mut partial_sum = 0;
        let mut prefix_sums = HashMap::<i32, u16>::with_capacity(nums.len() + 1);

        prefix_sums.insert(partial_sum, 1);

        for &number in nums.iter() {
            partial_sum += number;

            // partial_sum - prefix_sum = k

            if let Some(&count) = prefix_sums.get(&(partial_sum - k)) {
                total += count as i32;
            }

            prefix_sums
                .entry(partial_sum)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        total
    }
}

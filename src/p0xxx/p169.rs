fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 169
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm

        assert!(nums.len() >= 1);
        let mut majority = 0;
        let mut count = 0;

        for &number in nums.iter() {
            if count == 0 {
                majority = number;
                count = 1;
            } else if majority == number {
                count += 1;
            } else {
                count -= 1;
            }
        }

        majority
    }
}

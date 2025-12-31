impl Solution {
    // Problem 136

    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for number in nums.iter() {
            res ^= number;
        }

        res
    }
}

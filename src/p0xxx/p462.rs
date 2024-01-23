impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let nums = nums;
        let median0 = nums[nums.len() / 2];
        let mut moves = 0;
        for &n in &nums {
            moves += (n - median0).abs();
        }
        moves
    }
}


struct Solution {}

fn main() {
    println!("Hello World");
}
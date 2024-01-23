fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 283
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // Find the position of the first zero
        let hole = (0..nums.len()).filter(|&s| nums[s] == 0).next();

        if let Some(mut hole) = hole {
            // iterate numbers following the first zero
            for idx in hole + 1..nums.len() {
                if nums[idx] != 0 {
                    // swap number with the moving hole
                    nums.swap(hole, idx);
                    hole += 1;
                }
            }
        }
    }
    pub fn move_zeroes2(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&s| s != 0);
        nums.resize(len, 0);
    }
}

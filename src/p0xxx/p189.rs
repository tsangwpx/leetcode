fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 189
impl Solution {
    pub fn rotate3(nums: &mut Vec<i32>, k: i32) {
        panic!("It may be implemented using GCD of k and nums?")
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let len = nums.len();
        // Start:
        // [ 1 2 ... k ...  N-k ... N]
        // End:
        // [ N-k+1 ... N-k .. N 1 ... k ... N - k]
        //   ^................^ = k items
        //                      ^...........^ = (N - k) items

        // 1. reverse the whole Vec
        // [ N N-1 ... N-k ... k ... 2 1 ]
        nums.reverse();

        // 2. reverse first K items
        // [ N-k+1 N-k+2 .. N-1 N N-k ... k .. 2 1]
        //   ^------------------^ total k items between N-k+1 and N
        nums[0..k].reverse();

        // 3. reverse the rest N - k items
        // [ N-k+1 N-k+2 .. N-1 N 1 2 ... k ... N - k]
        nums[k..len].reverse();
    }

    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.rotate_right(k as usize)
    }
}

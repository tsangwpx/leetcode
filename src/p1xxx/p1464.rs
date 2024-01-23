fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 1464
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::with_capacity(2);
        heap.push(Reverse(nums[0]));
        heap.push(Reverse(nums[1]));

        for &value in nums.iter().skip(2) {
            let mut peek_mut = heap.peek_mut().unwrap();

            if value > peek_mut.0 {
                *peek_mut = Reverse(value);
            }
        }

        match heap.into_vec()[..] {
            [a, b] => (a.0 - 1) * (b.0 - 1),
            _ => unreachable!(),
        }
    }
}

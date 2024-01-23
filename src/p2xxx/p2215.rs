mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2215
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let set1 = nums1.into_iter().collect::<HashSet<i32>>();
        let set2 = nums2.into_iter().collect::<HashSet<i32>>();

        let ans0 = set1.difference(&set2).map(|&s| s).collect::<Vec<_>>();
        let ans1 = set2.difference(&set1).map(|&s| s).collect::<Vec<_>>();

        vec![ans0, ans1]
    }
}

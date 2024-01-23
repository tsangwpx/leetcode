fn main() {
    println!("Hello, world!");
}

struct Solution {}

/// Problem 300
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // Find longest strictly increasing subsequence with binary search
        // The idea is that each possible subsequence forms a linked link of numbers
        // We could replace these subseqneces with a single seq which maintain the ordering.
        // because only the longest length of such sequence is interested.
        // See also: https://en.wikipedia.org/wiki/Longest_increasing_subsequence

        use std::cmp::Ordering;

        assert!(nums.len() >= 1);

        let mut seq = Vec::with_capacity(nums.len());
        seq.push(nums[0]);

        for &number in nums.iter().skip(1) {
            match number.cmp(seq.last().unwrap()) {
                Ordering::Equal => continue,
                Ordering::Greater => seq.push(number),
                Ordering::Less => match seq.binary_search(&number) {
                    Ok(_) => {}
                    Err(idx) => {
                        // The number is bounded by the last item in seq
                        // The index < the seq length
                        seq[idx] = number
                    }
                },
            }
        }

        seq.len() as i32
    }
}

// Problem 31
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order

        // find the largest index k such nums[k] < nums[k + 1]
        let k = (0..nums.len())
            .rev()
            .skip(1)
            .filter(|&s| nums[s] < nums[s + 1])
            .next();

        match k {
            Some(k) => {
                assert!(k + 1 < nums.len());

                // Find the largest index h such that nums[h] > nums[k]
                // the lower bound is k + 1 because of prev conditon nums[k] < nums[k + 1]
                let h = (k + 1..nums.len())
                    .rev()
                    .filter(|&h| nums[h] > nums[k])
                    .next()
                    .unwrap();

                nums.swap(h, k);

                // Reverse nums[k+1..n]
                nums[k + 1..].reverse()
            }
            None => {
                // nums is the last permutation
                // reverse it become the first one and become completely sorted
                nums.reverse()
            }
        }
    }
}

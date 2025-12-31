// Problem 2099
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let k = k as usize;
        let mut heap = BinaryHeap::new();

        for (idx, value) in nums.iter().copied().enumerate() {
            if heap.len() < k {
                heap.push((Reverse(value), idx));
            } else {
                if let Some(mut peek) = heap.peek_mut() {
                    let (Reverse(top), _) = *peek;
                    if top < value {
                        *peek = (Reverse(value), idx);
                    }
                }
            }
        }

        let mut pairs = heap.into_vec();
        pairs.sort_unstable_by_key(|&(_, idx)| idx);

        pairs
            .into_iter()
            .map(|(Reverse(value), _)| value)
            .collect::<Vec<_>>()
    }
}

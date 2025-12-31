// Problem 373
impl Solution {
    pub fn k_smallest_pairs(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;

        let swapped = if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
            true
        } else {
            false
        };

        assert!(nums1.len() >= 1);
        assert!(nums2.len() >= 1);
        assert!(k >= 1);

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut res = Vec::with_capacity(k);
        let mut heap = BinaryHeap::with_capacity(nums1.len());
        heap.extend(
            nums1
                .iter()
                .copied()
                .map(|s| (Reverse(s + nums2[0]), 0usize)),
        );

        while res.len() < k && !heap.is_empty() {
            let mut top = heap.peek_mut().unwrap();

            let sum = top.0.0;
            let right_idx = top.1;
            let right = nums2[right_idx];
            let left = sum - right;

            if swapped {
                res.push(vec![right, left]);
            } else {
                res.push(vec![left, right]);
            }

            let next_idx = right_idx + 1;

            if next_idx == nums2.len() {
                drop(top);

                heap.pop();
            } else {
                let sum = sum - right + nums2[next_idx];
                *top = (Reverse(sum), next_idx);
            }
        }

        res
    }
}

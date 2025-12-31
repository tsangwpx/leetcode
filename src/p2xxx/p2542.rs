// Problem 2542
/**
 * You are given two 0-indexed integer arrays nums1 and nums2 of equal
 * length n and a positive integer k. You must choose a subsequence of
 * indices from nums1 of length k.
 *
 * For chosen indices i0, i1, ..., ik - 1, your score is defined as:
 *
 * 1. The sum of the selected elements from nums1 multiplied with the
 *      minimum of the selected elements from nums2.
 * 2. It can defined simply as:
 *      (nums1[i0] + nums1[i1] +...+ nums1[ik - 1])
 *      * min(nums2[i0] , nums2[i1], ... ,nums2[ik - 1]).
 *
 * Return the maximum possible score.
 *
 * A subsequence of indices of an array is a set that can be derived
 * from the set {0, 1, ..., n-1} by deleting some or no elements.
 */
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        // math:
        // dS = S_0 * db + b_1 * (a_1 - a_0)
        // dS: change of score
        // S_0: original score
        // db: change of minimum of b
        // b_1: change of the new minimum of b
        // a_1 and a_0: new and removed values respectively
        // If we iterate b from large to small, minimum of b is non-increasing, i.e. db <= 0.
        // So the first term of dS is non-positive
        // In order to maximize dS, the only way is to maximize  (a_1 - a_0) or minimze a_0
        // which tells us remove the smallest a_0.
        // Above is how the problem is solved by restructing min{b} is decreasing

        let k = k as usize;
        assert!(k >= 1);
        assert!(nums1.len() == nums2.len());
        assert!(nums1.len() >= k);

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut pairs = nums1.into_iter().zip(nums2.into_iter()).collect::<Vec<_>>();
        pairs.sort_unstable_by_key(|&(_, b)| Reverse(b));

        // argsort
        let mut best_score = 0i64;
        let mut heap_sum = 0i64;
        let mut heap = BinaryHeap::with_capacity(k);

        for (a, b) in pairs.iter().copied() {
            if heap.len() < k {
                // Filling the heap until k entries
                heap.push(Reverse(a));
                heap_sum += a as i64;

                if heap.len() == k {
                    best_score = best_score.max(heap_sum * b as i64);
                }

                continue;
            }

            // here we must replace the top of the heap no matter their values.
            // otherwise, the math above would be broken.

            let mut peek = heap.peek_mut().unwrap();

            heap_sum -= peek.0 as i64;
            heap_sum += a as i64;
            *peek = Reverse(a);
            best_score = best_score.max(heap_sum * b as i64);
        }

        best_score
    }
}

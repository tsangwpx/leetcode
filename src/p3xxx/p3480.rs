// Problem 3480
impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        /*
         * Observations
         * 1. Count subarrays, not subsequences
         * 2. Fix the right endpoint of subarrays, and see the smallest possible left
         * 3. Remember the left and increase it accordingly
         * 4. The second smallest possible left is also required to compute the gain after removing one pair
         * 5. The difference between smallest and second smallest is the gain after removing one pair
         */

        let n = n as usize;
        let mut events = vec![vec![]; n + 1];
        for pairs in conflicting_pairs.into_iter() {
            let &[a, b] = pairs.as_slice() else {
                unreachable!()
            };
            let (a, b) = (a.min(b) as usize, a.max(b) as usize);
            events[b].push(a);
        }

        let mut count = 0i64;
        let mut left_most = 0;
        let mut left_second = 0;
        let mut extra = vec![0; n + 1];

        for right in 1..=n {
            for left in events[right].iter().copied() {
                (left_most, left_second) = (left_most, left_second)
                    .max((left_most, left))
                    .max((left, left_most));
            }

            count += (right - left_most) as i64;
            extra[left_most] += (left_most - left_second) as i64;
            // println!("{:?} {} {}", (left_most, left_second), right, count);
        }
        // println!("{:?}", extra);

        count + extra.iter().max().copied().unwrap()
    }
}

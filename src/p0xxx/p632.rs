// Problem 632
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        #[derive(Debug, Clone, Copy)]
        struct Interval(i32, i32);

        #[inline]
        fn is_smaller(x: Interval, y: Interval) -> bool {
            let dx = x.1 - x.0;
            let dy = y.1 - y.0;
            dx.cmp(&dy).then(x.0.cmp(&y.0)).is_le()
        }

        let mut maximum = -100_000;
        let mut interval = Interval(-100_000, 100_000);

        let mut pq = Vec::with_capacity(nums.len());
        for (idx, row) in nums.iter().enumerate() {
            maximum = maximum.max(row[0]);
            pq.push((Reverse(row[0]), idx, 1));
        }

        let mut pq = BinaryHeap::from(pq);

        while let Some(mut top) = pq.peek_mut() {
            let (Reverse(minimum), seq_idx, item_idx) = *top;

            let new_interval = Interval(minimum, maximum);
            if is_smaller(new_interval, interval) {
                interval = new_interval;
            }

            if item_idx >= nums[seq_idx].len() {
                return vec![interval.0, interval.1];
            }

            let number = nums[seq_idx][item_idx];
            maximum = maximum.max(number);
            *top = (Reverse(number), seq_idx, item_idx + 1);
        }

        unreachable!()
    }
}

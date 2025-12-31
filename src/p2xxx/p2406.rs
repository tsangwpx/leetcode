// Problem 2406
impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        intervals.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut pq = BinaryHeap::new();

        for interval in intervals.iter() {
            let right = interval[1];
            let left = interval[0];

            if pq.peek().map(|&Reverse(s)| s < left).unwrap_or(false) {
                *pq.peek_mut().unwrap() = Reverse(right);
            } else {
                pq.push(Reverse(right));
            }
        }

        pq.len() as i32
    }
}

// Problem 57
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        assert!(new_interval.len() == 2);

        let mut a = new_interval[0];
        let mut b = new_interval[1];

        // bisect left
        let left = intervals
            .binary_search_by(|s| {
                let stop = unsafe { s.get_unchecked(1) };
                stop.cmp(&a).then(Ordering::Greater)
            })
            .unwrap_or_else(std::convert::identity);

        // bisect right
        let right = left
            + intervals[left..]
                .binary_search_by(|s| {
                    let start = unsafe { s.get_unchecked(0) };
                    start.cmp(&b).then(Ordering::Less)
                })
                .unwrap_or_else(std::convert::identity);

        // println!("{}: [{}, {}]", intervals.len(), left, right,);

        assert!(left <= right && right <= intervals.len());

        if left == right {
            // If same, the new interval is not covered
            intervals.insert(left, new_interval);
        } else {
            // Otherwise, right - left >= 1, and
            // left is inclusive and right is exclusive
            // so modify left inplace
            // and drop things between left + 1 and right (exclusive)

            a = a.min(intervals[left][0]);
            b = b.max(intervals[right - 1][1]);

            intervals[left][0] = a;
            intervals[left][1] = b;

            intervals.drain(left + 1..right);
        }

        intervals
    }
}

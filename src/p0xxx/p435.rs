// Problem 435
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() <= 1 {
            return 0;
        }

        intervals.sort_unstable_by_key(|s| s[1]);

        // Since intervals are sorted by their right endpoints
        // When we iterate the sorted intervals,
        // a new interval is placed if it is non-overlapping.
        //
        // For example,
        // the first non-overlapping interval ends in `intervals[0][1]`,
        // the second non-overlapping intervals ends in `intervals[i][1]`
        // such that intervals[i][0] >= intervals[0][1]
        //
        // Note that overlapping means measure of intersection is non-zero.
        //
        // The maximum number of non-overlapping intervals is found
        // The total number of intervals is subtracted by this number
        // to obtain the answer

        // finding the max number of non-overlapping interval groups
        let mut count = 1;
        let mut stop = intervals[0][1];

        for i in 1..intervals.len() {
            if intervals[i][0] >= stop {
                stop = intervals[i][1];
                count += 1;
            }
        }

        intervals.len() as i32 - count
    }
}

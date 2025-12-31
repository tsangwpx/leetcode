// Problem 452
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|interval| interval[1]);

        let mut arrows = 1;
        let mut right = points[0][1];

        for interval in points.iter() {
            if interval[0] > right {
                arrows += 1;
                right = interval[1];
            }
        }

        arrows
    }

    pub fn find_min_arrow_shots2(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|interval| (interval[0], interval[1]));

        let mut arrows = 0;
        let mut idx = 0;

        'outer: while idx < points.len() {
            // so far idx-th ballon is the leftmost ballon not burst by any arrow
            arrows += 1;

            // the rightmost point we can pick
            let mut right = points[idx][1];
            idx += 1;

            while idx < points.len() {
                if right < points[idx][0] {
                    // next ballon is out of the range
                    continue 'outer;
                }

                // maybe we should shrink the range of burst
                right = right.min(points[idx][1]);
                idx += 1;
            }
        }

        arrows
    }
}

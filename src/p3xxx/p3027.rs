// Problem 3027
impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points
            .into_iter()
            .map(|point| {
                if point.len() >= 2 {
                    (point[0], point[1])
                } else {
                    panic!("bad size");
                }
            })
            .collect::<Vec<(i32, i32)>>();

        points.sort_by_key(|&(x, y)| (x, -y));

        let mut count = 0;

        for i in 0..points.len() {
            let (_x0, y0) = points[i]; //top-left
            let mut ymax = i32::MIN;

            'inner: for j in i + 1..points.len() {
                let (_x1, y1) = points[j]; // bottom-right

                // now, we have x0 <= x1 due to sorting
                // the second point (x1, y1) must be vertically lower than the first point (x0, y0)
                // we need y1 <= y0
                // if x are the same, y is sorted from top to bottom
                // ymax record the largest y valid so far.
                // such that (x0, y1) and (?, ymax) is valid

                if y1 <= y0 && ymax < y1 {
                    count += 1;
                    ymax = y1;
                    if y0 == y1 {
                        break 'inner;
                    }
                }
            }
        }

        count
    }
}

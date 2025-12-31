// Problem 812
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        // Shoelace formula
        // x1 x2 x3 x1
        // y1 y2 y3 y1
        // = (x1y2 - x2y1) + (x2y3 - x3y2) + (x3y1 - x1y3)
        // = x1(y2 - y3) + x2(y3 -y1) + x3(y1 - y2)

        let mut max = 0.0f64;

        for i in 0..points.len() {
            let &[xi, yi] = points[i].as_slice() else {
                panic!();
            };

            for j in i + 1..points.len() {
                let &[xj, yj] = points[j].as_slice() else {
                    panic!();
                };

                for k in j + 1..points.len() {
                    let &[xk, yk] = points[k].as_slice() else {
                        panic!();
                    };

                    max = max
                        .max((xi * (yj - yk) + xj * (yk - yi) + xk * (yi - yj)).abs() as f64 * 0.5);
                }
            }
        }

        max
    }
}

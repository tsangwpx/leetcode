// Problem 3623
impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut counter = HashMap::<i32, i32>::new();

        for point in points.iter() {
            let &[_x, y] = point.as_slice() else {
                panic!();
            };

            *counter.entry(y).or_default() += 1;
        }

        let mut total_sides = 0;
        let mut res = 0;
        let mut max_count = 0;

        for (_, &count) in counter.iter() {
            let count = count as i64;
            max_count = max_count.max(count);

            let my_sides = count * (count - 1) / 2;
            total_sides += my_sides;
        }

        for (_, &count) in counter.iter() {
            let count = count as i64;
            let my_sides = count * (count - 1) / 2;
            let other_sides = total_sides - my_sides;
            let subtotal = my_sides * other_sides;

            res += subtotal;
            res %= MOD;
        }

        // println!(" {} {}", res, res / 2);
        if res & 1 == 1 {
            res += MOD;
            res /= 2;
            // res %= MOD;
        } else {
            res /= 2;
        }

        res as i32
    }
}

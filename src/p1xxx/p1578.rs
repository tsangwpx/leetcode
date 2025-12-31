// Problem 1578
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        // This is a dynamic programming problem.
        // And can be optimized to O(N) time & O(1) space
        let mut min_time = 0i32;

        let mut i = 0;

        while i < colors.len() {
            let start = i;
            let ch = colors.bytes().nth(i).unwrap();
            i += 1;

            while i < colors.len() && colors.bytes().nth(i).unwrap() == ch {
                i += 1;
            }
            let stop = i;
            let count = stop - start;

            if count >= 2 {
                let (time_sum, time_max) = needed_time.iter().skip(start).take(count).fold(
                    (0, 0),
                    |(time_sum, time_max), &time_need| {
                        (time_sum + time_need, time_max.max(time_need))
                    },
                );

                min_time += time_sum - time_max;
            }
        }

        min_time
    }
}

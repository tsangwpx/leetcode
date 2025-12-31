// Problem 2110
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut start = 0;

        for (idx, item) in prices.iter().copied().enumerate() {
            let is_smoothly_decreasing = idx + 1 < prices.len() && item - 1 == prices[idx + 1];

            // println!("{} {} {}", idx, is_smoothly_decreasing, start);

            if !is_smoothly_decreasing {
                let len = (idx + 1 - start) as i64;

                let count = (len + 1) * len / 2;
                // println!("len {} {}", len, count);
                res += count;

                start = idx + 1;
            }
        }

        res
    }
}

// Problem 2280
impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() <= 1 {
            return 0;
        }

        stock_prices.sort_unstable_by_key(|s| s[0]);

        let mut count = 1;

        #[inline]
        fn dxdy(stock_prices: &Vec<Vec<i32>>, i: usize) -> (i64, i64) {
            let dy = stock_prices[i][1] as i64 - stock_prices[i - 1][1] as i64;
            let dx = stock_prices[i][0] as i64 - stock_prices[i - 1][0] as i64;

            (dx, dy)
        }

        for i in 1..(stock_prices.len() - 1) {
            let (dx0, dy0) = dxdy(&stock_prices, i);
            let (dx1, dy1) = dxdy(&stock_prices, i + 1);

            // println!("{}: {}", i, dy0 as f64 / dx0 as f64);

            // if i == stock_prices.len().wrapping_sub(2) {
            //     println!("{}: {}", i + 1, dy1 as f64 / dx1 as f64);
            // }

            if dy1 * dx0 != dy0 * dx1 {
                count += 1;
            }
        }

        count
    }
}

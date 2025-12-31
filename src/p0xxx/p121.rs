// Problem 121
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // only ONE buy and one sell can be made.

        assert!(prices.len() >= 1);

        let mut lowest_seen = i32::MAX;
        let mut profit_max = 0;

        for price in prices.iter() {
            lowest_seen = lowest_seen.min(*price);
            profit_max = profit_max.max(price - lowest_seen);
        }

        profit_max
    }
}

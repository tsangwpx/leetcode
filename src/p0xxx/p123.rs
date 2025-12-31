// Problem 123
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut first_trans = vec![0; prices.len()];
        let mut lowest = prices[0];
        let mut profit_max = 0;

        for (i, &price) in prices.iter().enumerate().skip(1) {
            let profit = price - lowest;
            profit_max = profit_max.max(profit);
            first_trans[i] = profit_max;
            lowest = lowest.min(price);
        }

        let mut second_trans = vec![0; prices.len()];
        let mut highest = prices.last().copied().unwrap();
        for (i, &price) in prices.iter().enumerate().rev().skip(1) {
            second_trans[i] = highest - price;
            highest = highest.max(price);
        }

        let mut profit_max = 0;

        for i in 0..prices.len() {
            profit_max = profit_max.max(first_trans[i] + second_trans[i]);
        }

        profit_max
    }
}

// Problem 3573
impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;

        const MINIMUM: i64 = i32::MIN as i64;

        // profits[i] = max profit after completing i-th transaction
        let mut profits = vec![MINIMUM; k + 1];

        // sold[i] = max profits when holding SOLD in (i+1)-th transaction
        let mut sold = vec![MINIMUM; k];

        // bought[i] = max profit when holding BOUGHT in (i+1)-th transaction
        let mut bought = vec![MINIMUM; k];

        profits[0] = 0;

        for (day, price) in prices.iter().copied().enumerate() {
            let price = i64::from(price);

            for i in (0..k).rev() {
                profits[i + 1] = profits[i + 1].max(sold[i] - price).max(bought[i] + price);
                sold[i] = sold[i].max(profits[i] + price);
                bought[i] = bought[i].max(profits[i] - price);
            }

            // println!("Day {} Price {}", day + 1, price);
            // println!("{:?}", profits);
            // println!("{:?}", sold);
            // println!("{:?}", bought);
        }

        profits.iter().copied().max().unwrap()
    }
}

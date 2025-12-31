// Problem 714
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut owned = -prices[0] - fee;
        let mut unowned = 0;

        for price in prices.iter().skip(1).copied() {
            let owned2 = owned.max(unowned - price - fee);
            let unowned2 = unowned.max(owned + price);
            owned = owned2;
            unowned = unowned2;
        }

        unowned
    }
}

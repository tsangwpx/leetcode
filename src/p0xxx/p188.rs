// Problem 188
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        assert!(prices.len() >= 1);

        use std::mem::swap;

        let k = k as usize;
        assert!(k >= 1);

        const HOLD: usize = 0;
        const SOLD: usize = 1;

        let mut yesterday = vec![[0, 0]; k];
        let mut today = yesterday.clone();

        // initializaton
        // We may do all k transaction in the first day.
        // The balance is -prices[0] if hold and 0 if sold.
        for tran in 0..k {
            today[tran][HOLD] = -prices[0];
        }

        // skip the first day. because it is in the init phase
        for (idx, &price) in prices.iter().enumerate().skip(1) {
            swap(&mut today, &mut yesterday);

            let _day = idx + 1;

            // the first buy may be cheaper
            today[0][HOLD] = yesterday[0][HOLD].max(-price);

            // th first sell may be more profitable
            today[0][SOLD] = yesterday[0][SOLD].max(yesterday[0][HOLD] + price);

            // Sequential buys and sells may be cheaper and more profitable respectively.
            // They also depend on the state in the yesterday
            for tid in 1..k {
                let _transaction_count = tid + 1;

                // the (tid+1)-th buy depend on tid-th sells before today
                today[tid][HOLD] = yesterday[tid][HOLD].max(yesterday[tid - 1][SOLD] - price);

                // the (tid+1)-th sell depends on (tid+1)-th buy before today
                today[tid][SOLD] = yesterday[tid][SOLD].max(yesterday[tid][HOLD] + price);
            }
            // println!("{}: {:?}", idx, today);
        }

        today.iter().map(|s| s[SOLD]).max().unwrap()
    }
}

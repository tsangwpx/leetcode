// Problem 322
impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        assert!(amount >= 0);
        if amount == 0 {
            return 0;
        }

        // Why cions[i] may be as large as 2 ** 31 - 1??
        coins.retain(|&s| s <= amount);
        coins.sort();
        coins.dedup();

        if coins.len() == 0 || coins[0] > amount {
            return -1;
        } else if *coins.last().unwrap() == amount {
            return 1;
        }

        const NOT_FOUND: u16 = 10u16.pow(4) + 123;

        let mut dp = vec![NOT_FOUND; amount as usize + 1];
        for &coin in coins.iter() {
            dp[coin as usize] = 1;
        }

        for base in coins[0]..amount {
            let base_count = dp[base as usize];
            if base_count >= NOT_FOUND {
                continue;
            }

            for &coin in coins.iter() {
                let target = base + coin;

                if target > amount {
                    // coins are sorted
                    break;
                }
                let target_count = dp[target as usize];
                dp[target as usize] = target_count.min(base_count + 1);
            }
        }

        if dp[amount as usize] >= NOT_FOUND {
            -1
        } else {
            dp[amount as usize] as i32
        }
    }
}

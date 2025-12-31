// Problem 3652
impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let k = k as usize;
        assert!(strategy.len() == n);

        let base = (0..n).fold(0i64, |acc, i| {
            acc + i64::from(prices[i]) * i64::from(strategy[i])
        });

        let mut maximum = base;
        let mut delta = 0i64;
        for i in 0..n {
            match strategy[i] {
                1 => {}
                0 => delta += i64::from(prices[i]),
                -1 => delta += i64::from(prices[i]) * 2,
                _ => unreachable!(),
            }

            if i >= k / 2 {
                let j = i - k / 2;
                delta -= i64::from(prices[j]);
            }

            if i >= k {
                let j = i - k;
                match strategy[j] {
                    1 => delta += i64::from(prices[j]),
                    0 => {}
                    -1 => delta -= i64::from(prices[j]),
                    _ => unreachable!(),
                }
            }

            if i + 1 >= k {
                maximum = maximum.max(base + delta);
                // println!("{} {} {} {}", i, delta, base + delta, maximum);
            }
        }

        maximum
    }
}

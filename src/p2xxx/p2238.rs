// Problem 2238
// https://leetcode.com/problems/count-the-number-of-ideal-arrays/solutions/2265366/sieve-of-eratosthenes-o-maxvalue/

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        use std::sync::LazyLock;

        const MOD: i64 = 10i64.pow(9) + 7;

        const LIMIT: usize = 10001;
        const NUMBERS: usize = 14;

        static PRECOMPUTED: LazyLock<(Vec<Vec<i32>>, Vec<Vec<i32>>)> = LazyLock::new(|| {
            let mut ncr = vec![vec![1; NUMBERS]; LIMIT];
            let mut count = vec![vec![0; NUMBERS]; LIMIT];

            for n in 1..LIMIT {
                let size = (n + 1).min(NUMBERS);

                let (first, second) = ncr.split_at_mut(n);
                let prev = first.last().unwrap();
                let row = second.first_mut().unwrap();

                for r in 1..n.min(size) {
                    row[r] = ((prev[r - 1] as i64 + prev[r] as i64) % MOD) as i32;
                }

                // println!("{:?}", ncr[n]);
            }

            for div in 1..LIMIT {
                count[div][0] += 1;

                for i in (div..LIMIT).step_by(div).skip(1) {
                    let mut bars = 0;
                    while count[div][bars] != 0 {
                        count[i][bars + 1] += count[div][bars];
                        bars += 1;
                    }
                }

                // println!("{:?}", count[number]);
            }

            (ncr, count)
        });

        let (ncr, count) = &*PRECOMPUTED;
        let mut res = 0i64;

        for i in 1..=max_value as usize {
            let mut bars = 0;
            while bars < NUMBERS.min(n as usize) && count[i][bars] != 0 {
                res += ncr[n as usize - 1][bars] as i64 * count[i][bars] as i64;
                res %= MOD;

                bars += 1;
            }
        }

        res as i32
    }
}

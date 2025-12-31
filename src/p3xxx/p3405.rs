// Problem 3405
impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        let n = n as i64;
        let m = m as i64;
        let k = k as i64;

        fn combo(fact: &Vec<i32>, n: i64, r: i64) -> i64 {
            if r < 0 || r > n {
                return 0;
            }

            let r = r.min(n - r);

            if r == 0 || n == 0 {
                1
            } else if r == 1 {
                n
            } else {
                let mut res = 1;
                res *= fact[n as usize] as i64;
                res *= inverse(fact[r as usize] as i64);
                res %= MOD;
                res *= inverse(fact[(n - r) as usize] as i64);
                res %= MOD;
                res
            }
        }

        let fact = {
            let mut fact = vec![1i32; n as usize + 1];
            for i in 2..fact.len() {
                fact[i] = (fact[i - 1] as i64 * i as i64 % MOD) as i32;
            }
            fact
        };

        fn inverse(n: i64) -> i64 {
            powmod(n, MOD - 2)
        }

        fn powmod(base: i64, exp: i64) -> i64 {
            if exp < 0 {
                return 0;
            }

            let mut res = 1i64;
            let mut mul = base % MOD;
            let mut exp = exp;

            while exp != 0 {
                if (exp & 1) == 1 {
                    res *= mul;
                    res %= MOD;
                }

                mul *= mul;
                mul %= MOD;
                exp >>= 1;
            }

            if res < 0 {
                res += MOD;
            }

            res
        }

        let slots = n - k;

        if slots <= 0 {
            // impossible
            0
        } else {
            let mut res = m;
            res *= powmod(m - 1, n - k - 1);
            res %= MOD;
            res *= combo(&fact, n - 1, k);
            res %= MOD;
            res as _
        }
    }
}

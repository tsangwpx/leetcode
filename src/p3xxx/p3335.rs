// Problem 3335
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        // fn("a", 1) -> "b"
        // fn("z", 1) -> "ab"
        // fn("z", 2) -> "bc"
        // fn("z", 25) -> "yz"
        // fn("z", 26) -> "zab"
        // fn("z", 27) -> "abbc"

        type Matrix = [[i64; 26]; 26];

        // Fast-transform
        let mut ft = Matrix::default();

        for i in 0..26 {
            if i < 25 {
                ft[i][i + 1] = 1;
            } else {
                ft[i][0] = 1;
                ft[i][1] = 1;
            }
        }

        fn step(ft0: Matrix) -> Matrix {
            let mut ft1 = Matrix::default();

            for i in 0..26 {
                for j in 0..26 {
                    for k in 0..26 {
                        ft1[i][j] += (ft0[i][k] * ft0[k][j]) % MOD;
                    }

                    ft1[i][j] %= MOD;
                }
            }

            ft1
        }

        let mut dp0 = s.bytes().fold([0i64; 26], |mut acc, ch| {
            acc[(ch - b'a') as usize] += 1;
            acc
        });

        for shift in 0..=32 {
            if (t >> shift) == 0 {
                break;
            }

            if (t >> shift) & 1 == 1 {
                let mut dp1 = [0i64; 26];
                for i in 0..26 {
                    for j in 0..26 {
                        dp1[i] += (dp0[j] * ft[j][i]) % MOD;
                    }

                    dp1[i] %= MOD;
                }

                dp0 = dp1;
                // println!("res: {:?}", res);
            }

            ft = step(ft);
        }

        (dp0.iter().sum::<i64>() % MOD) as i32
    }
}

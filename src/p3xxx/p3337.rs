// Problem 3337
impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        const MOD: i32 = 10i32.pow(9) + 7;

        // fn("a", 1) -> "b"
        // fn("z", 1) -> "ab"
        // fn("z", 2) -> "bc"
        // fn("z", 25) -> "yz"
        // fn("z", 26) -> "zab"
        // fn("z", 27) -> "abbc"

        type Matrix = [[i32; 26]; 26];

        // Fast-transform
        let mut ft = Matrix::default();
        assert!(nums.len() == 26);

        for i in 0..26 {
            let follow = nums[i] as usize;
            for j in 0..follow {
                ft[i][(i + j + 1) % 26] = 1;
            }
        }

        #[inline]
        fn step(ft0: Matrix) -> Matrix {
            let mut ft1 = Matrix::default();

            for i in 0..26 {
                for j in 0..26 {
                    let mut tmp = 0i128;
                    for k in 0..26 {
                        tmp += i128::from(i64::from(ft0[i][k]) * i64::from(ft0[k][j]));
                    }

                    ft1[i][j] = (tmp % MOD as i128) as i32;
                }
            }

            ft1
        }

        let mut dp0 = s.bytes().fold([0i32; 26], |mut acc, ch| {
            acc[(ch - b'a') as usize] += 1;
            acc
        });

        for shift in 0..=32 {
            if (t >> shift) == 0 {
                break;
            }

            if (t >> shift) & 1 == 1 {
                let mut dp1 = [0i32; 26];
                for i in 0..26 {
                    let mut tmp = 0i128;

                    for j in 0..26 {
                        tmp += i128::from(i64::from(dp0[j]) * i64::from(ft[j][i]));
                    }

                    dp1[i] = (tmp % MOD as i128) as i32;
                }

                dp0 = dp1;
                // println!("res: {:?}", res);
            }

            ft = step(ft);
        }

        (dp0.iter().copied().map(i64::from).sum::<i64>() % MOD as i64) as i32
    }
}

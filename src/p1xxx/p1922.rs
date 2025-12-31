// Problem 1922
impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        let mut count = 1i64;
        let mut cap = 1;
        let mut acc = 1;

        while cap <= n {
            if cap == 1 {
                // even index: 0, 2, 4, 6, 8
                acc *= 5;
            } else if cap == 2 {
                // odd index: 2, 3, 5, 7
                acc *= 4;
            } else {
                acc *= acc;
                acc %= MOD;
            }

            if cap & n != 0 {
                count *= acc;
                count %= MOD;
            }

            cap <<= 1;
        }

        count as i32
    }
}

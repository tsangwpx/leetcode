// Problem 1513
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        let mut count = 0;
        let mut ones = 0;

        for ch in s.bytes().chain([b'0']) {
            if ch == b'0' {
                if ones >= 1 {
                    count += ones * (1 + ones) / 2;
                    count %= MOD;
                }

                ones = 0;
            } else if ch == b'1' {
                ones += 1;
            }
        }

        count as i32
    }
}

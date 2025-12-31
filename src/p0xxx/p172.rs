// Problem 172
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut coun5 = 0;
        let mut div5 = 5;
        let mut exp5 = 1;

        // trailing zeros are factor of 10
        // 10 = 2 * 5
        // there is one 5 in multiple of 5
        // two 5 in multiple of 25
        // three 5 in multiple of 125
        // so on and so forth

        loop {
            coun5 += n / div5 * exp5;
            coun5 -= (n / div5) * (exp5 - 1);

            if let Some(next_div5) = div5.checked_mul(5) {
                if next_div5 > n {
                    break;
                }
                div5 = next_div5;
                exp5 += 1;
            } else {
                panic!("Overflow!");
            }
        }

        // 2 is much more than 5
        let count2 = n / 2;
        count2.min(coun5)
    }
}

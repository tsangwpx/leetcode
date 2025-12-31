// Problem 1317
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn is_no_zero(num: i32) -> bool {
            if num == 0 {
                return false;
            }

            let mut rem = num;

            while rem != 0 {
                let (q, r) = (rem / 10, rem % 10);

                if r == 0 {
                    return false;
                }

                rem = q;
            }

            true
        }

        for a in 1..i32::MAX {
            if !is_no_zero(a) {
                continue;
            }

            let b = n - a;
            if !is_no_zero(b) {
                continue;
            }

            return vec![a, b];
        }

        unreachable!();
    }
}

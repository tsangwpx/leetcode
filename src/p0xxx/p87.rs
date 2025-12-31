// Problem 87
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();

        const MAX_LEN: usize = 30;

        type Memo = [[[Option<bool>; MAX_LEN]; MAX_LEN]; MAX_LEN];

        let mut dp: Memo = Memo::default();

        const ZERO: [i8; 26] = [0i8; 26];

        fn is_scramble(
            dp: &mut Memo,
            s1: &[u8],
            s2: &[u8],
            idx1: usize,
            idx2: usize,
            len: usize,
        ) -> bool {
            if len == 0 {
                return true;
            }

            if let Some(result) = dp[idx1][idx2][len - 1] {
                return result;
            }

            let mut balance = ZERO;

            for ch in s1.iter().skip(idx1).take(len).copied() {
                balance[(ch - b'a') as usize] += 1;
            }

            for ch in s2.iter().skip(idx2).take(len).copied() {
                balance[(ch - b'a') as usize] -= 1;
            }

            let mut result = false;

            if balance != ZERO {
                result = false;
            } else if len == 1 || len == 2 {
                result = true;
            } else {
                for left_len in 1..len {
                    let right_len = len - left_len;
                    if is_scramble(dp, s1, s2, idx1, idx2, left_len)
                        && is_scramble(dp, s1, s2, idx1 + left_len, idx2 + left_len, right_len)
                    {
                        result = true;
                        break;
                    }

                    if is_scramble(dp, s1, s2, idx1, idx2 + len - left_len, left_len)
                        && is_scramble(dp, s1, s2, idx1 + left_len, idx2, right_len)
                    {
                        result = true;
                        break;
                    }
                }
            }

            println!("{} {} {} {}", idx1, idx2, len, result);

            dp[idx1][idx2][len - 1] = Some(result);
            result
        }

        is_scramble(&mut dp, &s1, &s2, 0, 0, s1.len().min(s2.len()))
    }
}

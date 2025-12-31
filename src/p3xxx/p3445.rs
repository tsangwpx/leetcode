// Problem 3445
impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        /*
         * a is the char with odd count
         * b is the char with even count
         *
         * freq(s[i..j], a) - freq(s[i..j], b)
         * = (freq(s[..j], a) - freq(s[..i], a)) - (freq(s[..j], b) - freq(s[..i], b))
         * = (freq(s[..j], a) - freq(s[..j], b)) - (freq(s[..i], a) - freq(s[..i], b))
         *
         * freq(s[i..j], a) is odd
         * => freq(s[..j], a) and freq(s[..i], a) are in different parity.
         *
         * freq(s[i..j], b) is even
         * => freq(s[..j], b) and freq(s[..i], b) are in the same parity.
         *
         * freq(j, a) is odd, then freq(i, a) is even
         * freq(j, a) is odd, then freq(i, a) is even
         * freq(j, b) is even, then freq(i, b) is even
         *
         *
         * both endpoint indices are inclusive.
         * left index is inclusive and right index is exclusive in implementation.
         *
         * ref:
         * https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-ii/solutions/6831146/easy-to-understand-explanation-for-editorial-solution/
         */

        const CHARS: [u8; 5] = [b'0', b'1', b'2', b'3', b'4'];

        #[inline]
        fn parity_index(pair: (bool, bool)) -> usize {
            match pair {
                (false, false) => 0,
                (false, true) => 1,
                (true, false) => 2,
                (true, true) => 3,
            }
        }

        #[inline]
        fn prev_pair(pair: (bool, bool)) -> (bool, bool) {
            match pair {
                (true, true) => (false, true),
                (true, false) => (false, false),
                (false, true) => (true, true),
                (false, false) => (true, false),
            }
        }

        let mut res = i32::MIN;

        for a in CHARS {
            for b in CHARS {
                if a == b {
                    continue;
                }

                let mut ai_count = 0;
                let mut aj_count = 0;
                let mut bi_count = 0;
                let mut bj_count = 0;

                let mut i = 0;
                let mut memo = [i32::MAX; 4];

                for (j, ch) in s.bytes().enumerate() {
                    if ch == a {
                        aj_count += 1;
                    }
                    if ch == b {
                        bj_count += 1;
                    }

                    // extend s[..i] such that j + 1 - i >= k
                    // b is the char with even count in substring
                    //
                    while j + 1 - i >= k as usize && bj_count - bi_count >= 2 {
                        // save the state before eating next character
                        let meme_idx = parity_index((ai_count % 2 == 1, bi_count % 2 == 1));
                        memo[meme_idx] = memo[meme_idx].min(ai_count - bi_count);

                        let prev = s.bytes().nth(i).unwrap();
                        i += 1;

                        if prev == a {
                            ai_count += 1;
                        }

                        if prev == b {
                            bi_count += 1;
                        }
                    }

                    let prev_pair = prev_pair((aj_count % 2 == 1, bj_count % 2 == 1));
                    let prev_idx = parity_index(prev_pair);

                    if memo[prev_idx] != i32::MAX {
                        res = res.max(aj_count - bj_count - memo[prev_idx]);
                    }
                }
            }
        }

        res
    }
}

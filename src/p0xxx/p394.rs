use std::ops::ControlFlow;

// Problem 394
impl Solution {
    pub fn decode_string(encoded: String) -> String {
        #[inline]
        fn is_digit(ch: u8) -> bool {
            (b'0'..=b'9').contains(&ch)
        }

        // Find the next digit or close bracket or end of string
        #[inline]
        fn find_break(s: &str) -> usize {
            s.bytes()
                .position(|ch| is_digit(ch) || ch == b']')
                .unwrap_or(s.len())
        }

        let mut consumed = 0;
        let mut stack = vec![];
        let mut out = vec![];
        out.reserve(encoded.len());

        while consumed < encoded.len() {
            let ch = encoded.bytes().nth(consumed).unwrap();

            if is_digit(ch) {
                let digit_len = encoded[consumed..]
                    .bytes()
                    .position(|s| s == b'[')
                    .expect("Missing opening bracket");

                let count = str::parse::<usize>(&encoded[consumed..][..digit_len])
                    .expect("Bad repeat count");

                consumed += digit_len;
                consumed += 1; // conumsed "["

                // save the start position where the copy written to
                stack.push((out.len(), count));
            } else if ch == b']' {
                consumed += 1;

                // restore the repeat start position
                let (repeat_start, repeat_count) = stack.pop().unwrap();
                let repeat_stop = out.len();

                if repeat_count > 1 {
                    out.reserve((repeat_stop - repeat_start) * (repeat_count - 1));

                    for _ in 1..repeat_count {
                        out.extend_from_within(repeat_start..repeat_stop);
                    }
                }
            } else {
                let copy_len = find_break(&encoded[consumed..]);
                assert!(copy_len >= 1);

                out.reserve(copy_len);
                out.extend_from_slice(encoded[consumed..][..copy_len].as_bytes());

                consumed += copy_len;
            }
        }

        // SAFETY: All characters are guaranteed to be ASCII
        unsafe { String::from_utf8_unchecked(out) }
    }

    pub fn decode_string2(encoded: String) -> String {
        #[inline]
        fn is_digit(ch: u8) -> bool {
            (b'0'..=b'9').contains(&ch)
        }

        // Find the next digit or close bracket or end of string
        #[inline]
        fn find_break(s: &str) -> usize {
            s.bytes()
                .position(|ch| is_digit(ch) || ch == b']')
                .unwrap_or(s.len())
        }

        fn repeat(out: &mut Vec<u8>, encoded: &str) -> usize {
            // println!("repeat: {}", encoded);
            let bracket = encoded
                .bytes()
                .position(|s| s == b'[')
                .expect("Missing opening bracket");

            let count = str::parse::<usize>(&encoded[..bracket]).expect("Bad repeat count");
            let mut consumed = bracket + 1;

            // save the start position where the copy written to
            let repeat_start = out.len();
            let nbytes_copied = copy(out, &encoded[consumed..]);
            consumed += nbytes_copied;

            if encoded.bytes().nth(consumed).expect("Bad consumed count") != b']' {
                panic!("Expect closing bracket");
            }
            consumed += 1;

            // save the stop position where the copy written to (exclusive)
            let repeat_stop = out.len();

            if count > 1 {
                // repeat the last copy (count - 1) times
                out.reserve((repeat_stop - repeat_start) * (count - 1));

                for _ in 1..count {
                    out.extend_from_within(repeat_start..repeat_stop);
                }
            }

            consumed
        }

        fn copy(out: &mut Vec<u8>, encoded: &str) -> usize {
            // println!("copy: {}", encoded);
            let mut consumed = 0;

            while consumed < encoded.len() {
                let ch = encoded.bytes().nth(consumed).unwrap();

                if is_digit(ch) {
                    consumed += repeat(out, &encoded[consumed..]);
                } else {
                    let copy_len = find_break(&encoded[consumed..]);

                    if copy_len == 0 {
                        // may be ']' or end of the input
                        break;
                    }

                    out.reserve(copy_len);
                    out.extend_from_slice(encoded[consumed..][..copy_len].as_bytes());
                    consumed += copy_len;
                }
            }

            consumed
        }

        let mut out = vec![];

        let consumed = copy(&mut out, &encoded);

        // SAFETY: All characters are guaranteed to be ASCII
        unsafe { String::from_utf8_unchecked(out) }
    }
}

use std::intrinsics::fabsf32;

impl Solution {
    pub fn is_interleave(mut s1: String, mut s2: String, s3: String) -> bool {
        if s1.len() < s2.len() {
            std::mem::swap(&mut s1, &mut s2);
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        assert!(s1.len() <= 100);
        assert!(s2.len() <= 100);
        assert!(s1.len() <= s3.len());
        assert!(s2.len() <= s3.len());

        let mut dp0 = [false; 128];
        let mut dp1 = [false; 128];

        dp1[0] = true;
        for (j, &b) in s2.iter().enumerate() {
            assert!(j < s3.len());
            dp1[j + 1] = dp1[j] && b == s3[j];
        }

        for (i, &a) in s1.iter().enumerate() {
            std::mem::swap(&mut dp0, &mut dp1);

            assert!(i < s3.len());

            dp1[0] = dp0[0] && a == s3[i];

            for (j, &b) in s2.iter().enumerate() {
                assert!(i + j + 1 < s3.len());
                assert!(j + 1 < dp0.len());
                assert!(j + 1 < dp1.len());

                let ch = s3[i + j + 1];
                dp1[j + 1] = (dp0[j + 1] && a == ch) || (dp1[j] && b == ch);
            }
        }

        assert!(s2.len() < dp1.len());
        dp1[s2.len()]
    }

    pub fn is_interleave2(mut s1: String, mut s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        if s1.len() < s2.len() {
            std::mem::swap(&mut s1, &mut s2);
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut stack = Vec::with_capacity(s2.len());

        loop {
            println!("{} {} {} {:?}", i, j, k, stack);


            if k < s3.len() {
                let cmp1 = i < s1.len() && s1[i] == s3[k];
                let cmp2 = j < s2.len() && s2[j] == s3[k];

                if cmp1 && cmp2 {
                    let terminator = s3[k];
                    let mut c = 1;

                    while k + c < s3.len() && s3[k + c] != terminator
                        && i + c < s1.len() && s1[i + c] == s3[k + c]
                        && j + c < s2.len() && s2[j + c] == s3[k + c] {
                        c += 1;
                    }

                    stack.push((i, j + c, k + c));
                    i += c;
                    k += c;
                    continue;
                } else if cmp1 {
                    i += 1;
                    k += 1;
                    continue;
                } else if cmp2 {
                    j += 1;
                    k += 1;
                    continue;
                }
            }

            if i == s1.len() && j == s2.len() && k == s3.len() {
                return true;
            }

            match stack.pop() {
                Some((a, b, c)) => {
                    i = a;
                    j = b;
                    k = c;
                }
                None => return false,
            }
        }
    }
}

struct Solution {}

fn main() {
    println!("Hello World");
}
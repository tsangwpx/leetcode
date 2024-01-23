// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 91
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.starts_with('0') || s.len() == 0 {
            return 0;
        }

        let s = s.as_bytes();

        // after the first character, dp0 = dp1 = 1
        let mut dp0 = 1;
        let mut dp1 = 1;
        let mut last = s[0];
        println!("{}: {} {} {}", 0, 0, dp0, dp1);

        for (idx, &curr) in s.iter().enumerate().skip(1) {
            let dp2 = match (last, curr) {
                (b'0', b'0') => return 0,
                (b'0', b'1'..=b'9') => dp1,
                (b'1', b'0') => dp0,
                (b'1', b'1'..=b'9') => dp0 + dp1,
                (b'2', b'0') => dp0,
                (b'2', b'1'..=b'6') => dp0 + dp1,
                (b'2', b'7'..=b'9') => dp1,
                (b'3'..=b'9', b'0') => return 0,
                (b'3'..=b'9', b'1'..=b'9') => dp1,
                _ => panic!("{}: {:?}", idx, (last, curr)),
            };

            println!("{}: {} {} {}", idx, dp0, dp1, dp2);

            last = curr;
            dp0 = dp1;
            dp1 = dp2;
        }

        dp1
    }
}

// Problem 93
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut tmp = String::new();
        let mut res = vec![];

        fn is_valid(s: &str) -> bool {
            match s.len() {
                1 => true,
                2 => s.chars().nth(0) != Some('0'),
                3 => match s.chars().nth(0).unwrap() {
                    '1' => true,
                    '2' => match s.chars().nth(1).unwrap() {
                        '0'..='4' => true,
                        '5' => matches!(s.chars().nth(2).unwrap(), '0'..='5'),
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            }
        }

        fn generate(res: &mut Vec<String>, tmp: &mut String, s: &str, dots: usize) {
            if dots == 0 {
                if is_valid(s) {
                    let mut addr = tmp.clone();
                    addr.push_str(s);
                    res.push(addr);
                }
            } else {
                for stop in 1..=3.min(s.len()) {
                    let segment = &s[0..stop];
                    if is_valid(segment) {
                        let len = tmp.len();
                        tmp.push_str(segment);
                        tmp.push('.');
                        generate(res, tmp, &s[stop..], dots - 1);
                        tmp.truncate(len);
                    }
                }
            }
        }

        generate(&mut res, &mut tmp, &s, 3);

        res
    }
}

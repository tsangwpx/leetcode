// Problem 2264
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut found = false;
        let mut max_digit = 0;
        let mut idx = 0;

        while idx < num.len() {
            let ch = num.bytes().nth(idx).unwrap();
            let start = idx;

            while num.bytes().nth(idx) == Some(ch) {
                idx += 1;
            }

            let len = idx - start;
            if len < 3 {
                continue;
            }

            found = true;

            let digit = ch;
            max_digit = max_digit.max(digit);
        }

        let mut res = String::new();

        if found {
            let ch = max_digit as char;
            res.extend([ch, ch, ch]);
        }

        res
    }
}

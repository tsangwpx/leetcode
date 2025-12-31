// Problem 3461
impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits = s.bytes().map(|s| s - b'0').collect::<Vec<u8>>();

        while digits.len() >= 3 {
            let tmp = digits
                .windows(2)
                .map(|s| (s[0] + s[1]) % 10)
                .collect::<Vec<u8>>();
            digits.clone_from(&tmp);
            println!("{:?}", digits);
        }

        digits.len() >= 2 && digits[0] == digits[1]
    }
}

// Problem 306
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        // The primary problem is that we dont know the length of the first two numbers
        // so we have to guess / brute force.

        #[inline]
        fn has_leading_zero(value: u64, repr: &str) -> bool {
            if repr.is_empty() {
                false
            } else if value == 0 {
                repr.len() >= 2
            } else {
                repr.chars().nth(0) == Some('0')
            }
        }

        fn is_additive_number(first: u64, second: u64, num: &str) -> bool {
            let sum = first + second;
            let repr = sum.to_string();

            // println!("{} {} {}", first, second, num);
            if !num.starts_with(&repr) {
                false
            } else {
                let num = &num[repr.len()..];
                num.len() == 0 || is_additive_number(second, sum, num)
            }
        }

        for len1 in 1..=num.len() {
            let first_num = &num[..len1];
            let num = &num[len1..];
            let Ok(first) = first_num.parse::<u64>() else {
                continue;
            };
            if has_leading_zero(first, first_num) {
                continue;
            }

            for len2 in 1..=num.len() {
                // small optimization may be done here.
                // The sum of lengths of first, second, and their sum must be
                // smaller or equal to the length of the input string

                let second_num = &num[..len2];
                let num = &num[len2..];
                let Ok(second) = second_num.parse::<u64>() else {
                    continue;
                };
                if has_leading_zero(second, second_num) {
                    continue;
                }

                if num.len() >= 1 && is_additive_number(first, second, num) {
                    return true;
                }
            }
        }

        false
    }
}

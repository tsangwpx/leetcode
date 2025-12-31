// Problem 2094
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let mut unique = HashSet::new();
        for i in 0..digits.len() {
            // a is the last digit
            let a = digits[i];
            if a % 2 == 1 {
                continue;
            }

            for j in 0..digits.len() {
                if j == i {
                    continue;
                }

                // b is the leading digit
                let b = digits[j];
                if b == 0 {
                    continue;
                }

                for k in 0..digits.len() {
                    if k == i || k == j {
                        continue;
                    }

                    // c is the middle digit
                    let c = digits[k];
                    let number = a + c * 10 + b * 100;
                    unique.insert(number);
                }
            }
        }

        let mut sorted = unique.into_iter().collect::<Vec<i32>>();
        sorted.sort_unstable();
        sorted
    }
}

// Problem 202
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;

        let mut seen = HashSet::<i32>::new();
        let mut number = n;

        loop {
            let mut sum = 0;
            while number > 0 {
                sum += (number % 10).pow(2);
                number /= 10;
            }

            if sum == 1 {
                return true;
            } else if !seen.insert(sum) {
                return false;
            }

            number = sum;
        }
    }
}

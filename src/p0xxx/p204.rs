use std::ffi::FromVecWithNulError;

// Problem 204
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut count = 0;
        let mut is_prime = vec![true; n];

        for i in 2..n {
            if !is_prime[i as usize] {
                continue;
            }

            count += 1;

            let mut m = 2;

            while i * m < n {
                is_prime[i * m] = false;
                m += 1;
            }
        }

        count
    }
}

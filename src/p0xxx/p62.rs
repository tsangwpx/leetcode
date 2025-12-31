// Problem 62
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }

        // there are (m + n - 2) steps required to reach the destinaton
        // (m - 1) of them moving down, D
        // (n - 1) of them moving right, R
        // there are possible A = (m + n - 2)_C_(m - 1) steps for moving down
        // so, A = (m + n - 2)! / (n - 1)! / (m - 1)!

        // rearrange the numbers such that m >= n
        let (m, n) = {
            let mut m = m - 1;
            let mut n = n - 1;
            if m < n {
                let t = m;
                m = n;
                n = t;
            }
            (m as u64, n as u64)
        };

        // Now, A = (m + n) / m! / n! and eliminate m!
        // A = [(m + n) / n] * [(m + n - 1) / (n - 1)] * ... * [(m + 2) / 2] * [(m + 1) / 1]

        let mut ans = 1u64;
        let mut denominator = 1u64;

        // start from small numbers
        // because the divisibility proof is based on mathematcal induction
        for i in 1..=n {
            if let Some(product) = ans.checked_mul(m + i) {
                ans = product;
            } else {
                ans = ans / denominator;
                ans = ans * (m + i);
                denominator = 1;
            }

            if let Some(product) = denominator.checked_mul(i) {
                denominator = product;
            } else {
                ans = ans / denominator;
                denominator = 1;
            }
        }

        ans = ans / denominator;

        ans as i32
    }
}

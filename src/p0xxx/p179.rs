// Problem 179
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.iter().all(|&s| s == 0) {
            return "0".to_owned();
        }

        assert!(nums.len() >= 1);

        let mut pairs = nums
            .into_iter()
            .map(|number| (number, number.to_string()))
            .collect::<Vec<_>>();

        // 10**9 is the maximum
        // its string presentation is of length 10
        const LEN: usize = 10 + 1;
        const fn compute_factors() -> [i64; LEN] {
            let mut result = [0i64; LEN];
            let mut factor = 1i64;
            let mut i = 0;
            while i < result.len() {
                result[i] = factor;
                factor *= 10;
                i += 1;
            }

            result
        }

        const FACTORS: [i64; LEN] = compute_factors();

        pairs.sort_unstable_by(|&(a, ref a_str), &(b, ref b_str)| {
            assert!(a_str.len() < LEN);
            assert!(b_str.len() < LEN);

            let a = i64::from(a);
            let b = i64::from(b);
            let ab = a * FACTORS[b_str.len()] + b;
            let ba = b * FACTORS[a_str.len()] + a;

            // The validity of the sorting can be proved by verify
            // the transitive relation a ~ b, and b ~ c imply a ~ c
            //
            // a ~ b: f(b)a + b > f(a)b + a
            //        a(f(b) - 1) > b(f(a) - 1)
            // b ~ c: b(f(c) - 1) > c(f(b) - 1)
            // Thus, a(f(c) - 1) > c(f(a) - 1)
            // a ~ c

            ab.cmp(&ba).reverse() // reverse sorting
        });

        use std::iter::FromIterator;
        String::from_iter(pairs.into_iter().map(|(_, s)| s))
    }
}

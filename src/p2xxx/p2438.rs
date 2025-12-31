// Problem 2438
impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = vec![];

        for shift in 0..31 {
            let bit = 1 << shift;
            if n & bit == bit {
                powers.push(bit);
            }
        }

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut res = vec![];

        // actually, we may pre-compute the prefix products, O(31)
        // and use the multiplicative inverse to get back the result. O(1)
        // or pre-compute the query table of size smaller than 31x31

        for query in queries.iter() {
            let &[left, right] = query.as_slice() else {
                unreachable!("bad format");
            };
            let left = left as usize;
            let right = right as usize;

            assert!(left < powers.len() && right < powers.len());

            let mut prod = 1i64;

            for idx in left..=right {
                prod *= i64::from(powers[idx]);
                prod %= MOD;
            }

            res.push(prod as i32);
        }

        res
    }
}

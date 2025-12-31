// Problem 2197
impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(mut u: i32, mut v: i32) -> i32 {
            // https://en.wikipedia.org/wiki/Binary_GCD_algorithm
            if u == 0 {
                return v;
            } else if v == 0 {
                return u;
            }

            let i = u.trailing_zeros();
            let j = v.trailing_zeros();
            let k = i.min(j);

            u >>= i;
            v >>= j;

            loop {
                if u > v {
                    std::mem::swap(&mut u, &mut v);
                }

                v -= u;

                if v == 0 {
                    return u << k;
                }

                v >>= v.trailing_zeros();
            }
        }

        let mut row = vec![];

        for item in nums.iter().copied() {
            row.push(item);

            while row.len() >= 2 {
                let u = row[row.len() - 2];
                let v = row[row.len() - 1];
                let factor = gcd(u, v);
                if factor == 1 {
                    break;
                }
                row.pop();
                row.pop();
                row.push(u / factor * v);
            }
        }

        row
    }
}

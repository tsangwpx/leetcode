// Problem 2929
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut count = 0;
        let imin = 0.max(n - limit * 2);
        let imax = n.min(limit);

        for i in imin..=imax {
            let jmin = 0.max(n - limit - i);
            let jmax = limit.min(n - i);

            count += i64::from(jmax - jmin + 1);
        }

        count
    }
}

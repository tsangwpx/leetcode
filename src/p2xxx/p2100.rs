// Problem 2100
impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let t = time as usize;
        let n = security.len();

        if t >= n {
            return vec![];
        } else if t == 0 {
            return (0..security.len() as i32).collect::<Vec<_>>();
        }

        let mut result = vec![true; n];

        result[0..t].fill(false);
        result[n - t..].fill(false);

        let mut monotonic_days = 0;

        for i in 1..security.len() {
            if security[i] <= security[i - 1] {
                monotonic_days += 1;
            } else {
                monotonic_days = 0;
            }

            result[i] = result[i] && monotonic_days >= time;
        }

        let mut monotonic_days = 0;
        for i in (0..security.len()).rev().skip(1) {
            if security[i] <= security[i + 1] {
                monotonic_days += 1;
            } else {
                monotonic_days = 0;
            }

            result[i] = result[i] && monotonic_days >= time;
        }

        result
            .into_iter()
            .enumerate()
            .filter_map(|(idx, ok)| ok.then_some(idx as i32))
            .collect::<Vec<i32>>()
    }
}

// Problem 3201
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 2;

        let patterns = [[0, 0], [0, 1], [1, 0], [1, 1]];
        let mut indices = [0usize; 4];
        let mut counts = [0; 4];

        for item in nums.iter().copied() {
            let rem = item % MOD;

            for (p, (i, c)) in patterns
                .iter()
                .zip(indices.iter_mut().zip(counts.iter_mut()))
            {
                if *i >= p.len() {
                    *i = 0;
                }

                if rem == p[*i] {
                    *i += 1;
                    *c += 1;
                }
            }
        }

        counts.iter().max().copied().unwrap_or(0)
    }
}

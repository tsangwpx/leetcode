// Problem 386v2
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut out = Vec::with_capacity(n as usize);

        fn dfs(n: i32, out: &mut Vec<i32>, base: i32) {
            let start = if base == 0 { 1 } else { base };
            let stop = (base + 9).min(n);

            for k in start..=stop {
                out.push(k);
                dfs(n, out, k * 10);
            }
        }

        dfs(n, &mut out, 0);

        out
    }
}

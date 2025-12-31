// Problem 1871
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        // This solution is slow. See solutions by others
        use std::collections::BTreeSet;

        let path = s.as_bytes();
        if path.last().copied().unwrap() == b'1' {
            return false;
        }

        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;

        let mut unused = s
            .bytes()
            .enumerate()
            .filter_map(|(idx, ch)| if ch == b'0' { Some(idx) } else { None })
            .collect::<BTreeSet<_>>();

        fn dfs(
            path: &[u8],
            min_jump: usize,
            max_jump: usize,
            unused: &mut BTreeSet<usize>,
            pos: usize,
        ) -> bool {
            let dst = path.len() - 1;
            let start = (pos + min_jump).min(path.len());
            let stop = (pos + max_jump + 1).min(path.len());

            if (start..stop).contains(&dst) {
                return true;
            }

            let nexts = unused.range(start..stop).copied().collect::<Vec<_>>();
            nexts.iter().copied().for_each(|s| {
                unused.remove(&s);
            });

            for next in nexts.into_iter().rev() {
                if dfs(path, min_jump, max_jump, unused, next) {
                    return true;
                }
            }

            false
        }

        dfs(path, min_jump, max_jump, &mut unused, 0)
    }
}

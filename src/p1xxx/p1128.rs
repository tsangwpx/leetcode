// Problem 1128
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let mut counter = HashMap::<(i32, i32), i32>::new();

        for pair in dominoes.iter() {
            let &[a, b] = pair.as_slice() else {
                continue;
            };

            *counter.entry((a.min(b), a.max(b))).or_default() += 1;
        }

        counter
            .values()
            .copied()
            .map(|s| s * (s - 1) / 2)
            .sum::<i32>()
    }
}

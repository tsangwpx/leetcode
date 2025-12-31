// Problem 2707
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut costs = vec![i32::MAX; s.len() + 1];
        costs[0] = 0;

        for idx in 0..s.len() {
            costs[idx + 1] = costs[idx + 1].min(costs[idx] + 1);

            for word in dictionary.iter() {
                if s[idx..].starts_with(word) {
                    let dst = idx + word.len();
                    costs[dst] = costs[dst].min(costs[idx]);
                }
            }
        }

        costs.last().copied().unwrap()
    }
}

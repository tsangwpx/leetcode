// Problem 2140
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0i64; questions.len()];
        let mut base = 0;
        let mut best_score = 0;

        for (idx, item) in questions.iter().enumerate() {
            let &[points, brainpower] = item.as_slice() else {
                unreachable!();
            };

            base = base.max(dp[idx]);
            let score = base + i64::from(points);
            best_score = best_score.max(score);

            println!("{} {} {} {}", idx, base, points, score);

            let idx2 = idx + brainpower as usize + 1;
            if idx2 < questions.len() {
                dp[idx2] = dp[idx2].max(score);
            }
        }

        best_score
    }
}

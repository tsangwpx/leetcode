// Problem 679
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let nums = cards.into_iter().map(f64::from).collect::<Vec<_>>();

        fn dfs(nums: &[f64]) -> bool {
            if nums.len() <= 1 {
                return if let Some(&item) = nums.first() {
                    (item - 24.0).abs() < 1.0e-5
                } else {
                    false
                };
            }

            let mut work = vec![];

            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    work.clear();

                    for k in 0..nums.len() {
                        if k == i || k == j {
                            continue;
                        }

                        work.push(nums[k]);
                    }

                    let a = nums[i];
                    let b = nums[j];

                    for value in [a + b, a - b, b - a, a / b, b / a, a * b] {
                        if value.is_nan() {
                            continue;
                        }

                        work.push(value);

                        if dfs(&work) {
                            return true;
                        }

                        work.pop();
                    }
                }
            }

            false
        }

        dfs(&nums)
    }
}

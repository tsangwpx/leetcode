// Problem 39
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn solve(
            result: &mut Vec<Vec<i32>>,
            buffer: &mut Vec<i32>,
            candidates: &Vec<i32>,
            offset: usize,
            remaining: i32,
        ) {
            if remaining < 0 {
                return;
            }

            if remaining == 0 {
                result.push(buffer.clone());
            }

            for index in offset..candidates.len() {
                let value = candidates[index];
                buffer.push(value);
                solve(result, buffer, candidates, index, remaining - value);
                buffer.pop();
            }
        }

        let mut res = Vec::with_capacity(150);
        let mut buffer = Vec::with_capacity(20);

        solve(&mut res, &mut buffer, &candidates, 0, target);

        res
    }
}

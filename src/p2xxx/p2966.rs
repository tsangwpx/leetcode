// Problem 2966
impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::ops::ControlFlow;
        nums.sort_unstable();

        match nums
            .chunks(3)
            .try_fold(Vec::with_capacity(nums.len() / 3), |mut res, chunk| {
                if chunk[2] - chunk[0] > k {
                    ControlFlow::Break(vec![])
                } else {
                    res.push(vec![chunk[0], chunk[1], chunk[2]]);
                    ControlFlow::Continue(res)
                }
            }) {
            ControlFlow::Continue(value) => value,
            ControlFlow::Break(value) => value,
        }
    }
}

// Problem 546
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        // explained by
        // https://leetcode.com/problems/remove-boxes/solutions/101310/java-top-down-and-bottom-up-dp-solutions/

        // A 3d array to store optimal subproblems
        // dp[i][j][k] is the max score
        // when deleting boxes[i..=j] with the weight of box[i] is (k + 1).

        // special test:
        // [1,2,2,1,2,2,1]
        // [1,2,2,2,2,1] -> 1
        // [1,1] -> 1 + 4 * 4
        // [] -> 1 + 4 * 4 + 2 * 2 = 21

        let mut dp = vec![vec![vec![0; boxes.len()]; boxes.len()]; boxes.len()];

        for i in 0..boxes.len() {
            for k in 0..boxes.len() {
                dp[i][i][k] = ((k + 1) * (k + 1)) as i32;
            }
        }

        for dist in 1..boxes.len() {
            // distance between first and last, (inclusive)
            // 0 1 2 3 4
            // ^ first ^ last => dist = 4 - 0 = 4, last = first + 4

            // when dist = boxes.len() - 1, first = 0 only
            // when dist = boxes.len() - 2, first = 0 or 1
            // when dist = 1, first = 0..boxes.len() - 1
            // when dist = 0, first = 0..boxes.len()
            // so first = 0..(boxes.len() - dist)

            for first in 0..(boxes.len() - dist) {
                let last = first + dist;

                for k in 0..first + 1 {
                    let mut max_point = ((k + 1) * (k + 1)) as i32 + dp[first + 1][last][0];

                    for m in (first + 1)..(last + 1) {
                        if boxes[first] == boxes[m] {
                            max_point = max_point.max(dp[first + 1][m - 1][0] + dp[m][last][k + 1]);
                        }
                    }

                    dp[first][last][k] = max_point;
                }
            }
        }

        dp[0].last().unwrap().first().copied().unwrap()
    }
}

use std::ffi::FromVecWithNulError;

// Problem 1335
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;

        if job_difficulty.len() < d {
            return -1;
        }

        assert!(d >= 1);
        assert!(job_difficulty.len() >= 1);

        let len = job_difficulty.len();

        // the first day is simply acc max
        let mut dp0 = job_difficulty
            .iter()
            .scan(0, |acc, item| {
                *acc = (*acc).max(*item);
                Some(*acc)
            })
            .collect::<Vec<_>>();
        let mut dp1 = vec![i32::MAX; len];

        assert!(dp0.len() == len && dp1.len() == len);

        let mut stack = Vec::with_capacity(job_difficulty.len());

        // in sequential day
        for day in 1..d {
            // masking the impossible slots (reduendant)
            dp1[day.saturating_sub(1)] = i32::MAX;
            dp1[day.saturating_sub(2)] = i32::MAX;

            stack.clear();

            for i in day..job_difficulty.len() {
                // jobs[i] is the last job this day
                //
                // dp1[i] = min_j { dp0[j] + max_k jobs[k] } where j < k <= i
                //
                // Say there are monotonic subsequence in job_difficulty
                //
                // Increasing: jobs[x]  <=  jobs[y] <=  jobs[i]
                //             dp0[x - 1]   dp0[y - 1]  dp0[i - 1]
                //
                // Keep jobs[i] is the biggest and pick the smallest possible dp0
                // dp1[i] = min_j { dp0[j] } + jobs[i] where (day - 1) <= j < i
                //
                // because dp1[y] have computed the minimum to up jobs[y]
                // we may reuse the result by substracting jobs[y] and add our jobs[i]
                // dp1[i] = dp1[y] - jobs[y] + jobs[i]
                //
                // Decreasing: jobs[x]  >   jobs[y] >   jobs[i]
                //             dp0[x - 1]   dp0[y - 1]  dp0[i - 1]
                //
                // We assume jobs[i] the smallest, and try to do more jobs with higher difficulty
                // this day, whose indices are stored in a stack.
                //
                // Luckily, we have computed dp1[i], which is in fact equal to dp1[x] = dp1[y]
                // So dp1[i] = dp1[y] which is the top of the decreasing stack
                //
                // this means:
                // dp1[i] = min(dp0[i - 1] + jobs[i], f(dp1, jobs))
                // dp1[i] depend on dp0[i - 1] + jobs[i] and dp1 itself

                dp1[i] = if i > 0 { dp0[i - 1] } else { 0 } + job_difficulty[i];

                while let Some(&j) = stack.last() {
                    // Make jobs[i] the smallest

                    if job_difficulty[i] < job_difficulty[j] {
                        break;
                    }

                    stack.pop();
                    dp1[i] = dp1[i].min(dp1[j] - job_difficulty[j] + job_difficulty[i]);
                }

                if let Some(&j) = stack.last() {
                    dp1[i] = dp1[i].min(dp1[j]);
                }

                stack.push(i);
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0.last().copied().unwrap()
    }

    pub fn min_difficulty3(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;

        if job_difficulty.len() < d {
            return -1;
        }

        assert!(d >= 1);
        assert!(job_difficulty.len() >= 1);

        let len = job_difficulty.len();

        // the first day is simply acc max
        let mut dp0 = job_difficulty
            .iter()
            .scan(0, |acc, item| {
                *acc = (*acc).max(*item);
                Some(*acc)
            })
            .collect::<Vec<_>>();
        let mut dp1 = vec![i32::MAX; len];

        assert!(dp0.len() == len && dp1.len() == len);

        // in sequential day
        for day in 1..d {
            // masking the impossible slots (reduendant)
            dp1[day.saturating_sub(1)] = i32::MAX;
            dp1[day.saturating_sub(2)] = i32::MAX;

            for i in day..job_difficulty.len() {
                // jobs[i] is the last job this day

                let mut today_max = job_difficulty[i];
                let mut total_min = i32::MAX;

                for j in (day - 1..i).rev() {
                    // jobs[j] is the last job last day

                    total_min = total_min.min(dp0[j] + today_max);

                    // since we are iterating backward, jobs[j] must be done today in next iteration
                    today_max = today_max.max(job_difficulty[j]);
                }

                dp1[i] = total_min;
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        match dp0.last().copied().unwrap() {
            i32::MAX => -1,
            s => s,
        }
    }

    pub fn min_difficulty2(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;

        if job_difficulty.len() < d {
            return -1;
        }

        assert!(d >= 1);
        assert!(job_difficulty.len() >= 1);

        let len = job_difficulty.len();

        // range_max[i][j] = max {jobs[k]} where i <= k <= j
        // only the top-right triangular is used
        let mut range_max = vec![vec![0; len]; len];

        // filling diagonal
        for i in 0..len {
            range_max[i][i] = job_difficulty[i];
        }

        // fill entries from left to right, then from bottom to top
        // A B C D
        //   E F G
        //     H I
        // order: I F G B C D
        for i in (0..len).rev() {
            for j in i + 1..len {
                range_max[i][j] = range_max[i + 1][j].max(range_max[i][j - 1]);
            }
        }

        // the first day is simply range_max[0]
        let mut dp0 = range_max[0].clone();
        let mut dp1 = vec![i32::MAX; len];

        assert!(dp0.len() == len && dp1.len() == len);

        // in sequential day
        for day in 1..d {
            // masking the impossible slots (reduendant)
            dp1[day.saturating_sub(1)] = i32::MAX;
            dp1[day.saturating_sub(2)] = i32::MAX;

            for i in day..job_difficulty.len() {
                // jobs[i] is the last job this day

                let mut difficulty = i32::MAX;

                for j in day - 1..i {
                    // jobs[j] is the last job last day

                    difficulty = difficulty.min(dp0[j] + range_max[j + 1][i]);
                }

                dp1[i] = difficulty;
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        match dp0.last().copied().unwrap() {
            i32::MAX => -1,
            s => s,
        }
    }
}

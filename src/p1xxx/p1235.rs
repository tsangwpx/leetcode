// Problem 1235
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        assert!(start_time.len() == end_time.len() && start_time.len() == profit.len());
        assert!(start_time.len() >= 1);

        let mut jobs = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|((start, end), profit)| (start, end, profit))
            .collect::<Vec<_>>();
        jobs.sort_unstable_by_key(|s| s.1);
        // println!("{:?}", jobs);

        let mut dp = vec![0; jobs.len()];

        for i in 0..jobs.len() {
            let (start_time, end_time, profit) = jobs[i];

            // best profit at or before start_time
            let start_idx = jobs.partition_point(|&(_, time, _)| time <= start_time);
            let start_profit = if start_idx > 0 { dp[start_idx - 1] } else { 0 };

            // We could eliminate the binary search below,
            // because we know the best profit so far.

            // best profit at or before end_time
            // we need to narrow the search range up to current job (exclusive)
            let end_idx = jobs[0..i].partition_point(|&(_, time, _)| time <= end_time);
            let end_profit = if end_idx > 0 { dp[end_idx - 1] } else { 0 };

            // the best profit at end_time is either wait or do this job
            let best_profit = end_profit.max(start_profit + profit);

            // println!("{}->{}: {}+{}", start_time, end_time, start_profit, profit);

            dp[i] = best_profit;
        }

        *dp.last().unwrap()
    }
}

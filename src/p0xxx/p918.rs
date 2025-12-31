// Problem 918
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        // acknowledge to lee215.
        //
        // The linaer case is known as Kadane's algorithm
        // In the circular case, the max subarray may cross the periodic boundary.
        // Say, "MMM mmm MMM", where MMMMMM is the max-sum subarray.
        // Instead of calculate sum(MMMMMMM) directly, min(mmm) is computed.
        // sum(MMMMMMM) = total sum - min(mmm)
        // Note that sum(MMM mmm MMM) = min(mmm) if all numbers are non-positive

        assert!(nums.len() >= 1);
        let n = nums.len();

        let mut total_sum = nums[0];
        let mut best_max_sum = nums[0]; // the best max sum
        let mut max_sum = nums[0]; // max sum so far

        let mut best_min_sum = nums[0]; // the best min sum
        let mut min_sum = nums[0]; // min sum so far

        for i in 1..n {
            total_sum += nums[i];

            max_sum = nums[i].max(max_sum + nums[i]);
            best_max_sum = best_max_sum.max(max_sum);

            min_sum = nums[i].min(min_sum + nums[i]);
            best_min_sum = best_min_sum.min(min_sum);
        }

        if best_min_sum == total_sum {
            // All numbers are negative
            best_max_sum
        } else {
            best_max_sum.max(total_sum - best_min_sum)
        }
    }

    pub fn max_subarray_sum_circular2(mut nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        let n = nums.len();
        nums.extend_from_within(..n - 1);

        let mut max_sum = nums[0]; // the max sum so far
        let mut sum = nums[0]; // the current sum
        let mut start = 0; // where the sum start

        for i in 1..nums.len() {
            // the sum should starts within N elements before.
            while i >= n + start {
                sum -= nums[start];
                start += 1;
            }
            // skip non-positive items
            while start < i && nums[start] <= 0 {
                sum -= nums[start];
                start += 1;
            }

            // println!("{}={}: start={}, sum={}+{}", i, i % n, start, sum, nums[i]);

            if sum >= 0 {
                // equality here is a must.
                // Otherwise, start is reset to a position too behind, which may skip possible larger sum.
                sum += nums[i];
            } else {
                // reset the sum and record this as start
                sum = nums[i];
                start = i;
            }
            max_sum = max_sum.max(sum);
        }

        println!("start={}, n={}", start, n);

        max_sum
    }
}

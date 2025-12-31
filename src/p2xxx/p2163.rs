// Problem 2163
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = nums.len() / 3;
        assert!(n >= 1 && n * 3 == nums.len());

        let mut result = vec![[0; 2]; n + 1];

        let mut sum_first = 0i64;
        let mut sum_second = 0i64;
        let mut max_heap = BinaryHeap::from_iter(nums.iter().copied().take(n).map(|s| {
            sum_first += i64::from(s);
            s
        }));

        result[0][0] = sum_first;

        for i in 0..n {
            let item = nums[n + i];
            max_heap.push(item);
            let popped = max_heap.pop().unwrap();
            sum_first += i64::from(item) - i64::from(popped);
            result[i + 1][0] = sum_first;
        }

        let mut min_heap = BinaryHeap::from_iter(nums[2 * n..].iter().copied().take(n).map(|s| {
            sum_second += i64::from(s);
            Reverse(s)
        }));
        result[n][1] = sum_second;

        for i in 0..n {
            let item = nums[n + n - i - 1];
            min_heap.push(Reverse(item));
            let popped = min_heap.pop().unwrap().0;
            sum_second += i64::from(item) - i64::from(popped);
            result[n - i - 1][1] = sum_second;
        }

        result.iter().fold(i64::MAX, |mindiff, &[first, second]| {
            mindiff.min(first - second)
        })
    }
}

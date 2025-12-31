// Problem 2873, Problem 2874
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut maximum = 0i64;
        let mut max_num = 0;
        let mut max_delta = 0;

        for number in nums.iter().copied().map(i64::from) {
            maximum = maximum.max(max_delta * number);
            max_delta = max_delta.max(max_num - number);
            max_num = max_num.max(number);
        }

        maximum
    }
}

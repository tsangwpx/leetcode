// Problem 1526
impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut prev = 0;

        for item in target.iter().copied() {
            if prev > item {
                res += prev - item;
            }

            prev = item;
        }

        res += prev;

        res
    }
}

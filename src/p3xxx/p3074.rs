// Problem 3074
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut remaining = apple.iter().copied().sum::<i32>();

        capacity.sort();
        let mut count = 0;

        while remaining > 0 {
            if let Some(box_) = capacity.pop() {
                count += 1;
                remaining -= box_;
            } else {
                break;
            }
        }

        count
    }
}

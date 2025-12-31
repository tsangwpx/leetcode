// Problem 3477
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut baskets = baskets;

        let mut count = 0;

        for quantity in fruits.iter().copied() {
            let idx = baskets.iter().position(|&s| s >= quantity);

            if let Some(idx) = idx {
                baskets.remove(idx);
            } else {
                count += 1;
            }
        }

        count
    }
}

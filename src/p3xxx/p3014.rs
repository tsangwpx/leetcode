// Problem 3014
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut remaining = word.len() as i32;
        let mut cost = 0;
        let mut hit = 1;

        while remaining > 0 {
            let used = if remaining >= 8 { 8 } else { remaining };
            remaining -= used;

            cost += hit * used;
            hit += 1;
        }

        cost
    }
}

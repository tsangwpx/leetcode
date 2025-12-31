// Problem 2501
impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        nums.sort();
        nums.dedup();

        let mut count = 0;
        let mut table = HashMap::<i64, i32>::new();

        for k in nums.into_iter().map(|s| i64::from(s)) {
            let kk = k * k;
            if let Some(kc) = table.get(&k).copied() {
                count = count.max(kc);
                let kkc = table.entry(kk).or_default();
                *kkc = (*kkc).max(kc + 1);
            } else {
                let kkc = table.entry(kk).or_default();
                *kkc = 2.max(*kkc);
            }
        }

        count
    }
}

// Problem 3494
impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        // TODO dp0 and dp1 can be combined into one
        // because there is no overlapping in access pattern.

        let n = skill.len();

        let mut dp0 = vec![0i64; skill.len()];
        let mut dp1 = dp0.clone();

        // compute the finish time by each wizard.
        for potion in mana.iter().copied() {
            for (i, factor) in skill.iter().copied().enumerate() {
                let cost = i64::from(potion) * i64::from(factor);

                // this wizard is working hard
                let mut done_by = dp0[i] + cost;

                if i >= 1 {
                    // wait on the last wizard in current potion
                    done_by = done_by.max(dp1[i - 1] + cost);
                }

                if i + 1 < n {
                    // wait the next wizard in previous potion
                    done_by = done_by.max(dp0[i + 1]);
                }

                dp1[i] = done_by;
            }

            // since there is no gap between wizard, fill the gap in reverse
            for (i, factor) in skill.iter().copied().enumerate().skip(1).rev() {
                let cost = i64::from(potion) * i64::from(factor);
                dp1[i - 1] = dp1[i] - cost;
            }

            // println!("{:?}", dp1);

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0.last().copied().unwrap()
    }
}

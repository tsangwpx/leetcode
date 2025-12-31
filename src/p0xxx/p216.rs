// Problem 216
/**
 * This solution builds the result bottom-up (form combinations)
 *
 * Or we may subtract the target and backtrace what is removed.
 *
 */
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        use std::cell::RefCell;
        use std::collections::HashMap;

        let k = k as usize;
        assert!(k <= 9);

        type Table = HashMap<i32, Vec<Vec<i32>>>;
        let k_sums = vec![RefCell::new(Table::new()); k + 1];

        for num in 1..=9 {
            if num > n {
                continue;
            }

            for k in (1..k).rev() {
                for (&sum, combinations) in k_sums[k].borrow().iter() {
                    if sum + num > n {
                        continue;
                    }

                    k_sums[k + 1]
                        .borrow_mut()
                        .entry(sum + num)
                        .or_default()
                        .extend(combinations.iter().map(|combo| {
                            let mut combo = combo.clone();
                            combo.push(num);
                            combo
                        }));
                }
            }

            k_sums[1]
                .borrow_mut()
                .entry(num)
                .or_default()
                .push(vec![num]);
        }

        let res = k_sums[k]
            .borrow_mut()
            .remove(&n)
            .or_else(|| Some(vec![]))
            .unwrap();

        res
    }
}

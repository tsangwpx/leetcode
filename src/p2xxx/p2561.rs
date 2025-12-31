// Problem 2561
impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        use std::collections::HashMap;

        #[inline]
        fn count_items(counter: &mut HashMap<i32, i32>, src: &[i32]) {
            for item in src.iter().copied() {
                *counter.entry(item).or_default() += 1;
            }
        }

        let mut counter1 = HashMap::new();
        count_items(&mut counter1, &basket1);

        let mut counter_total = counter1.clone();
        count_items(&mut counter_total, &basket2);

        let mut delta = vec![];
        let mut invalid_count = 0;
        let mut minimum_cost = i32::MAX;

        for (&cost, &total_count) in counter_total.iter() {
            if total_count % 2 == 1 {
                return -1;
            }

            let dst_count = total_count / 2;
            let cur_count = counter1.get(&cost).copied().unwrap_or(0);
            let change = dst_count - cur_count;
            invalid_count += i64::from(change.abs());

            if cost < minimum_cost {
                minimum_cost = cost;
            }

            if change != 0 {
                delta.push((cost, change));
            }
        }

        delta.sort_unstable_by_key(|&(s, _)| s);

        // println!("{} {}", minimum_cost, invalid_count);
        // println!("{:?}", delta);

        invalid_count /= 2;

        let double_min_cost = i64::from(minimum_cost) * 2;
        let mut res = 0i64;

        for (cost, change) in delta {
            let cost = i64::from(cost);
            let consumed = invalid_count.min(i64::from(change).abs());

            res += cost.min(double_min_cost) * consumed;
            invalid_count -= consumed;
            if invalid_count == 0 {
                break;
            }
        }

        res
    }
}

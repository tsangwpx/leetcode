// Problem 40
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let mut counter = HashMap::new();

        for number in candidates.iter().copied() {
            counter
                .entry(number)
                .and_modify(|count| *count += 1usize)
                .or_insert(1);
        }

        let mut partials = HashMap::new();
        partials.insert(0, vec![vec![]]);

        for (&number, &multiplicity) in counter.iter() {
            let mut change = HashMap::new();

            for (&base, values) in partials.iter() {
                for count in 1..=multiplicity {
                    let sum = base + number * (count as i32);

                    if sum > target {
                        break;
                    }

                    for row in values.iter() {
                        let mut row = row.clone();
                        row.extend(std::iter::repeat(number).take(count));
                        change.entry(sum).or_insert_with(|| vec![]).push(row);
                    }
                }
            }

            for (key, mut values) in change.drain() {
                partials
                    .entry(key)
                    .or_insert_with(|| vec![])
                    .extend(values.drain(..));
            }
        }

        partials.remove(&target).unwrap_or_else(|| vec![])
    }
}

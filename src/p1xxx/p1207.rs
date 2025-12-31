// Problem 1207
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let counter = arr
            .into_iter()
            .fold(HashMap::<i32, i16>::new(), |mut counter, number| {
                counter
                    .entry(number)
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
                counter
            });

        counter.values().copied().collect::<HashSet<i16>>().len() == counter.len()
    }
}

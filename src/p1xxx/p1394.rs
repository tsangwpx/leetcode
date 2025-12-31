// Problem 1394
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let counter = arr
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut map, &item| {
                *map.entry(item).or_default() += 1;
                map
            });

        counter.iter().fold(
            -1,
            |max, (&key, &value)| {
                if key == value { max.max(key) } else { max }
            },
        )
    }
}

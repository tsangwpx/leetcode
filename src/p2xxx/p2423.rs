// Problem 2423
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        use std::collections::HashMap;

        let mut counter = vec![0i32; 26];
        for ch in word.bytes() {
            counter[(ch - b'a') as usize] += 1;
        }

        counter.retain(|&s| s > 0);

        if counter.len() <= 1 {
            return true;
        }

        let mut table = HashMap::<i32, i32>::new();

        counter.iter().copied().for_each(|freq| {
            *table.entry(freq).or_default() += 1;
        });

        if table.len() == 1 && table.contains_key(&1) {
            return true;
        }

        if table.len() == 2 {
            let mut it = table.into_iter();
            let (freq0, count0) = it.next().unwrap();
            let (freq1, count1) = it.next().unwrap();

            if freq0 == 1 && count0 == 1 {
                return true;
            }

            if freq1 == 1 && count1 == 1 {
                return true;
            }

            if freq0 + 1 == freq1 {
                return count1 == 1;
            }

            if freq1 + 1 == freq0 {
                return count0 == 1;
            }
        }

        false
    }
}

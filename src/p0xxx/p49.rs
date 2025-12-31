// Problem 49
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut table = HashMap::new();

        fn make_key(s: &str) -> [u8; 26] {
            let mut key = [0u8; 26];

            for ch in s.bytes() {
                key[(ch - b'a') as usize] += 1;
            }

            key
        }

        for item in strs.into_iter() {
            let key = make_key(&item);

            table
                .entry(key) //
                .or_insert_with(|| vec![])
                .push(item);
        }

        table.into_values().collect()
    }
}

// Problem 318
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        // map hash to max len
        let mut table = HashMap::new();
        let mut max_product = 0;

        for word in words.iter() {
            let mut bits = 0u32;

            for ch in word.bytes() {
                bits |= 1 << (ch - b'a');
            }

            for (bits2, len2) in table.iter() {
                if bits & bits2 == 0 {
                    max_product = max_product.max(word.len() * len2);
                }
            }

            let len = table.entry(bits).or_default();
            *len = (*len).max(word.len());
        }

        max_product as i32
    }
}

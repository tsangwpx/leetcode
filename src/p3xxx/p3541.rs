// Problem 3541
impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let counter = s.bytes().fold([0u8; 128], |mut counter, ch| {
            if (ch as usize) < counter.len() {
                counter[ch as usize] += 1;
            }
            counter
        });

        let mut max_vowel_count = 0;
        let mut max_consonant_count = 0;

        for (ch, count) in counter.iter().copied().enumerate() {
            if matches!(ch as u8, b'a' | b'e' | b'i' | b'o' | b'u') {
                max_vowel_count = max_vowel_count.max(count);
            } else {
                max_consonant_count = max_consonant_count.max(count);
            }
        }

        (max_vowel_count + max_consonant_count) as i32
    }
}

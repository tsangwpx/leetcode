// Problem 2785
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        const VOWELS: &[u8; 10] = b"AEIOUaeiou";

        let mut pos = 0;
        let mut counter = [0; VOWELS.len()];

        for ch in s.bytes() {
            if let Some(idx) = VOWELS.iter().position(|&vowel| ch == vowel) {
                counter[idx] += 1;
            }
        }

        s.bytes()
            .map(|ch| {
                if let Some(_) = VOWELS.iter().position(|&vowel| ch == vowel) {
                    while pos < counter.len() {
                        if counter[pos] == 0 {
                            pos += 1;
                            continue;
                        }

                        counter[pos] -= 1;
                        return VOWELS[pos] as char;
                    }

                    unreachable!("extra vowel")
                } else {
                    ch as char
                }
            })
            .collect::<String>()
    }
}

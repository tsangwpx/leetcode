// Problem 1935
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_letter = broken_letters.bytes().fold([false; 26], |mut acc, ch| {
            let pos = (ch - b'a') as usize;
            if pos < 26 {
                acc[pos] = true;
            }
            acc
        });

        let mut word_count = 0;
        let mut good_word = true;

        for ch in text.bytes() {
            if ch == b' ' {
                if good_word {
                    word_count += 1;
                }

                good_word = true;
                continue;
            }

            let pos = (ch - b'a') as usize;
            if pos < 26 && broken_letter[pos] {
                good_word = false;
            }
        }

        if good_word {
            word_count += 1;
        }

        word_count
    }
}

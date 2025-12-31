// Problem 3330
impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut res = 1;

        let mut idx = 0;
        while idx < word.len() {
            let ch = word.bytes().nth(idx).unwrap();
            let mut multiple = 1;
            while word.bytes().nth(idx + multiple) == Some(ch) {
                multiple += 1;
            }

            idx += multiple;

            if multiple >= 2 {
                res += multiple as i32 - 1;
            }
        }

        res
    }
}

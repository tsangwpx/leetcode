impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by(|a, b| a.as_bytes().iter().rev().cmp(b.as_bytes().iter().rev()));

        let mut length = 0;
        let mut count = 0;

        for (i, w) in words.iter().enumerate() {
            if i + 1 == words.len() || !words[i + 1].ends_with(w.as_str()) {
                length += w.len();
                count += 1;
            }
        }

        (length + count) as i32
    }
}

fn main() {
    Solution::longest_palindrome("hello world".to_string());
}

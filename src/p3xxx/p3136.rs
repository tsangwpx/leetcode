// Problem 3136
impl Solution {
    pub fn is_valid(word: String) -> bool {
        word.len() >= 3
            && word
                .chars()
                .all(|ch| matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z'))
            && word
                .chars()
                .any(|ch| matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
            && word.chars().any(|ch| {
                ch.is_ascii_alphabetic()
                    && !matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
            })
    }
}

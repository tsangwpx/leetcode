// Problem 3227
impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.bytes()
            .any(|s| matches!(s, b'a' | b'e' | b'i' | b'o' | b'u'))
    }
}

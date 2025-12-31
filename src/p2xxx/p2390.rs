// Problem 2390
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result = String::new();

        for ch in s.chars() {
            if ch == '*' {
                result.pop();
            } else {
                result.push(ch);
            }
        }

        result
    }
}

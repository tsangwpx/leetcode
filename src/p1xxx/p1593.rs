// Problem 1593
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        use std::collections::HashSet;

        fn split<'a>(parts: &mut HashSet<&'a [u8]>, s: &'a [u8], start: usize) -> usize {
            if start == s.len() {
                parts.len()
            } else {
                let mut maximum = 0;
                for stop in (start + 1)..=s.len() {
                    let segment = &s[start..stop];

                    if parts.insert(segment) {
                        maximum = maximum.max(split(parts, s, stop));
                        parts.remove(segment);
                    }
                }

                maximum
            }
        }

        split(&mut HashSet::new(), &s.into_bytes(), 0) as i32
    }
}

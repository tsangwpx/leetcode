// Problem 316
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut last_occurrences = [usize::MAX; 26];

        for (idx, ch) in s.bytes().enumerate() {
            let offset = (ch - b'a') as usize;
            last_occurrences[offset] = idx;
        }

        let mut stack = vec![];

        for (idx, ch) in s.bytes().enumerate() {
            let offset = (ch - b'a') as usize;

            if !stack.contains(&ch) {
                while let Some(top) = stack.last().copied() {
                    let top_offset = (top - b'a') as usize;
                    println!("{} {}: {:?}", top as char, ch as char, stack);

                    if top > ch && idx < last_occurrences[top_offset] {
                        // if the top item is larger than current character
                        // and we still have chance to add it back
                        stack.pop();
                    } else {
                        break;
                    }
                }

                stack.push(ch);
            }
        }

        stack.into_iter().fold(String::new(), |mut s, ch| {
            s.push(ch as char);
            s
        })
    }
}

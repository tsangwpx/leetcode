// Problem 2434
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        // Count character frequencies
        let mut counter = [0; 26];
        s.bytes().for_each(|b| {
            counter[(b - b'a') as usize] += 1;
        });

        let mut paper = String::with_capacity(s.len());
        let mut t = String::with_capacity(s.len());

        let mut i = 0;

        // iterate the frequency counter so that the smallest possible character are searched
        for idx in 0..counter.len() {
            // if the stack contains the character smaller or equal to the current character
            // pop it back to the output
            while let Some(last) = t.bytes().last() {
                let idx2 = (last - b'a') as usize;
                if idx2 > idx {
                    break;
                }
                paper.push(t.pop().unwrap());
            }

            // keep searching the current smallest character, while maintaining the counter
            while counter[idx] > 0 && i < s.len() {
                let ch = s.bytes().nth(i).unwrap();
                let idx2 = (ch - b'a') as usize;
                i += 1;

                if idx2 == idx {
                    // add to output
                    paper.push(ch as char);
                    counter[idx] -= 1;
                } else {
                    // not the smallest, add to the stack
                    t.push(ch as char);
                    counter[idx2] -= 1;
                }
            }
        }

        paper.extend(t.chars().rev());
        paper
    }
}

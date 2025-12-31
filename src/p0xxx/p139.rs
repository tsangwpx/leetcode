// Problem 139
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        assert!(s.len() > 0);
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        let mut used = vec![false; s.len() + 1]; // One more space for simplicity
        used[0] = true;
        heap.push(0usize);

        'outer: while let Some(start) = heap.pop() {
            for word in word_dict.iter() {
                let dest = start + word.len();
                if dest >= used.len() || used[dest] {
                    // skip if the slot is attempted or in the heap.
                    continue;
                }

                if s.get(start..).map_or(false, |s| s.starts_with(word)) {
                    used[dest] = true;

                    if dest == s.len() {
                        // we could return true here
                        // but only one function exit seem more readable?
                        break 'outer;
                    }

                    heap.push(dest);
                }
            }
        }

        used[s.len()]
    }
}

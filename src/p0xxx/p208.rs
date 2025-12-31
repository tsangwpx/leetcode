// Problem 208

use std::collections::BTreeSet;

#[derive(Default)]
struct Trie {
    set: BTreeSet<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        self.set.insert(word);
    }

    fn search(&self, word: String) -> bool {
        self.set.contains(&word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        use std::ops::Bound;
        use std::ops::Bound::{Included, Unbounded};

        for item in BTreeSet::range::<String, (Bound<&String>, Bound<&String>)>(
            &self.set,
            (Included(&prefix), Unbounded),
        ) {
            return item.starts_with(&prefix);
        }

        false
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
fn dummy() {}

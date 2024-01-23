/// Untested! Not submit yet


use std::cmp::max;
use std::collections::BTreeMap;

pub struct WordFilter {
    root: Trie,
}

pub struct Trie {
    value: i32,
    children: BTreeMap<u8, Trie>,
}

impl Trie {
    fn new(value: i32) -> Self {
        Self { value, children: BTreeMap::new() }
    }
}

impl Trie {
    fn insert(&mut self, key: &[u8], val: i32) {
        self.value = max(self.value, val);
        if let Some((&ch, tail)) = key.split_first() {
            self.children
                .entry(ch)
                .or_insert_with(|| Trie::new(val))
                .insert(tail, val);
        }
    }

    fn find(&self, key: &[u8]) -> i32 {
        if let Some((&ch, tail)) = key.split_first() {
            self.children
                .get(&ch)
                .map(|s| s.find(tail))
                .unwrap_or(-1)
        } else {
            self.value
        }
    }
}


impl WordFilter {
    pub fn new(words: Vec<String>) -> Self {
        use std::collections::HashMap;

        let mut root = Trie::new(0);

        for (idx, word) in words.into_iter().enumerate() {
            let mut words = word.into_bytes();
            let word_len = words.len();
            words.reserve_exact(word_len + 1);
            words.push(b'~');
            words.extend_from_within(0..word_len);
            let words = words.as_slice();

            for start in 0..word_len {
                for stop in (word_len + 1)..words.len() {
                    root.insert(&words[start..stop], idx as i32);
                }
            }
        }

        Self { root }
    }


    pub fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut key = suffix.into_bytes();
        key.reserve_exact(prefix.len() + 1);
        key.push(b'~');
        key.extend_from_slice(prefix.as_bytes());
        let key = key.as_slice();
        self.root.find(key)
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

fn main() {
    let wf = WordFilter::new(vec!["apple".to_owned()]);
    wf.f("a".to_string(), "e".to_string());
}
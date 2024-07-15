mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

extern crate rand;

// Problem 3045
impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        use std::collections::HashMap;

        #[derive(Debug, Default)]
        struct TrieNode {
            terminal: i64,
            children: HashMap<(u8, u8), TrieNode>,
        }

        let mut root = TrieNode::default();
        let mut count = 0;

        fn insert(node: &mut TrieNode, word: &[u8], idx: usize) -> i64 {
            if idx >= word.len() {
                let result = node.terminal;
                node.terminal += 1;
                return result;
            }

            let first = word[idx];
            let last = word[word.len() - idx - 1];
            let child = node.children.entry((first, last)).or_default();
            let result = insert(child, word, idx + 1);
            node.terminal + result
        }

        for word in words {
            count += insert(&mut root, word.as_bytes(), 0);
            // println!("{:?}", root);
        }

        count
    }
}

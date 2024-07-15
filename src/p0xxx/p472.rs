mod leetcode_prelude;

use std::intrinsics::fabsf32;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 472
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        #[derive(Debug, Default)]
        struct Node {
            terminal: bool,
            children: [usize; 26],
        }

        words.sort_unstable_by_key(|s| s.len());

        let mut trie = Vec::<Node>::new();
        trie.push(Node::default());

        let mut res = vec![];

        fn add(trie: &mut Vec<Node>, node: usize, word: String, pos: usize) {
            // Add word to trie
            if pos >= word.len() {
                trie[node].terminal = true;
            } else {
                let ch = word.bytes().nth(pos).unwrap();
                let slot = (ch - b'a') as usize;
                let mut next = trie[node].children[slot];
                if next == 0 {
                    next = trie.len();
                    trie.push(Node::default());
                    trie[node].children[slot] = next;
                }

                add(trie, next, word, pos + 1);
            }
        }

        fn dfs(trie: &Vec<Node>, node: usize, line: &str) -> bool {
            if line.is_empty() {
                return trie[node].terminal;
            }

            if trie[node].terminal && dfs(trie, 0, line) {
                return true;
            }

            let ch = line.bytes().nth(0).unwrap();
            let slot = (ch - b'a') as usize;
            let next = trie[node].children[slot];

            next != 0 && dfs(trie, next, &line[1..])
        }

        for word in words.into_iter() {
            let concat = dfs(&trie, 0, &word);

            if concat {
                res.push(word);
            } else {
                add(&mut trie, 0, word, 0);
            }
        }

        res
    }
}

mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 140
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        #[derive(Debug, Default)]
        struct Node {
            word: Option<String>,
            children: [usize; 26],
        }

        let mut trie = Vec::<Node>::new();
        trie.push(Node::default());

        fn add(trie: &mut Vec<Node>, node: usize, word: String, pos: usize) {
            // Add word to trie
            if pos >= word.len() {
                trie[node].word = Some(word);
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

        for word in word_dict {
            add(&mut trie, 0, word, 0);
        }

        let mut res = vec![];
        let mut parts = vec![];

        fn bfs<'a>(
            trie: &'a Vec<Node>,
            node: usize,
            res: &mut Vec<String>,
            line: &str,
            parts: &mut Vec<&'a str>,
        ) {
            if let Some(word) = &trie[node].word {
                // Encounter a word, push the word into stack
                parts.push(&word);

                if line.is_empty() {
                    // if reach end of line, lets form a sentence.
                    res.push(parts.join(" "));
                } else {
                    // otherwise, restart from the trie root
                    bfs(trie, 0, res, line, parts);
                }

                parts.pop();

                // Don't return here because longer word may be found if characters are available.
            }

            if !line.is_empty() {
                let ch = line.bytes().nth(0).unwrap();
                let slot = (ch - b'a') as usize;
                let next = trie[node].children[slot];

                if next != 0 {
                    bfs(trie, next, res, &line[1..], parts);
                }
            }
        }

        bfs(&trie, 0, &mut res, &s, &mut parts);

        res
    }
}

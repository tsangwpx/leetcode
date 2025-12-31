// Problem 1032
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    terminal: bool,
    children: [u32; 26],
}

struct StreamChecker {
    trie: Vec<Node>,
    query: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = vec![];
        trie.reserve(words.len());
        trie.push(Node {
            terminal: false,
            children: [u32::MAX; 26],
        });

        fn insert(trie: &mut Vec<Node>, node: usize, word: &str, idx: usize) {
            let idx = idx.wrapping_sub(1);
            if idx >= word.len() {
                trie[node].terminal = true;
                return;
            } else {
                let ch = word.bytes().nth(idx).unwrap();
                let index = (ch - b'a') as usize;

                if trie[node].children[index] == u32::MAX {
                    trie[node].children[index] = trie.len() as u32;
                    trie.push(Node {
                        terminal: false,
                        children: [u32::MAX; 26],
                    });
                }

                insert(trie, trie[node].children[index] as usize, word, idx);
            }
        }

        for word in words {
            insert(&mut trie, 0, &word, word.len());
        }

        // println!("{:?}", trie);

        Self {
            trie,
            query: String::new(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.query.push(letter);

        let mut found = false;
        let mut node = 0;

        for ch in self.query.bytes().rev() {
            let idx = (ch - b'a') as usize;
            let next_node = self.trie[node].children[idx];
            if next_node == u32::MAX {
                break;
            }
            let next_node = next_node as usize;

            found |= self.trie[next_node].terminal;
            node = next_node;
        }

        found
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */
fn f() {}

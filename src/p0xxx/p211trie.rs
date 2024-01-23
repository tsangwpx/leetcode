mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 211

use std::cell::RefCell;
use std::rc::Rc;

struct WordDictionary {
    root: Rc<RefCell<TrieNode>>,
}

#[derive(Debug, Default)]
struct TrieNode {
    terminal: bool,
    children: [Option<Rc<RefCell<TrieNode>>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            root: Default::default(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = self.root.clone();

        for ch in word.bytes() {
            let idx = (ch - b'a') as usize;
            assert!(idx < 26);

            let mut node_inner = node.borrow_mut();

            let next_node = match &node_inner.children[idx] {
                None => {
                    let next_node: Rc<RefCell<TrieNode>> = Default::default();
                    node_inner.children[idx] = Some(next_node.clone());
                    next_node
                }
                Some(next_node) => next_node.clone(),
            };

            drop(node_inner);

            node = next_node;
        }

        node.borrow_mut().terminal = true;
    }

    fn search(&self, word: String) -> bool {
        fn search_node(root: &Rc<RefCell<TrieNode>>, word: &[u8]) -> bool {
            if word.len() == 0 {
                return root.borrow().terminal;
            }

            let mut range = match word[0] {
                b'.' => 0..26usize,
                ch @ b'a'..=b'z' => (ch - b'a') as usize..(ch - b'a' + 1) as usize,
                _ => unreachable!(),
            };

            range.any(|idx| match &root.borrow().children[idx] {
                None => return false,
                Some(node) => search_node(node, &word[1..]),
            })
        }

        search_node(&self.root, word.as_bytes())
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
fn dummy() {}

// Problem 1032

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::rc::Weak;

struct TrieNode {
    terminal: bool,
    children: [Option<Rc<RefCell<TrieNode>>>; 26],
}

struct StreamChecker {
    root: Rc<RefCell<TrieNode>>,
    queue: VecDeque<Weak<RefCell<TrieNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut root = Rc::new(RefCell::new(TrieNode {
            terminal: false,
            children: Default::default(),
        }));

        fn insert(node: &Rc<RefCell<TrieNode>>, word: &str, idx: usize) {
            let mut inner = RefCell::borrow_mut(&node);

            if idx >= word.len() {
                inner.terminal = true;
                return;
            } else {
                let ch = word.bytes().nth(idx).unwrap();
                let index = (ch - b'a') as usize;

                if inner.children[index].is_none() {
                    inner.children[index] = Some(Rc::new(RefCell::new(TrieNode {
                        terminal: false,
                        children: Default::default(),
                    })));
                }

                let next_node = inner.children[index].as_ref().unwrap();

                insert(next_node, word, idx + 1);
            }
        }

        for word in words {
            insert(&root, &word, 0);
        }

        Self {
            root,
            queue: VecDeque::new(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        let index = (letter as u8 - b'a') as usize;

        self.queue.push_back(Rc::downgrade(&self.root));

        let count = self.queue.len();
        let mut found = false;

        for _ in 0..count {
            let Some(node) = self.queue.pop_front().and_then(|s| s.upgrade()) else {
                continue;
            };
            let inner = RefCell::borrow(&node);
            if let Some(ref next_node) = inner.children[index] {
                found |= RefCell::borrow(&next_node).terminal;
                self.queue.push_back(Rc::downgrade(&next_node));
            }
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

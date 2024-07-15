mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {}

struct Solution {}

extern crate rand;

// Problem 140
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Trie {
    word: Option<Rc<String>>,
    children: [Option<Rc<RefCell<Trie>>>; 26],
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let root: Rc<RefCell<Trie>> = Default::default();

        fn add_word(node: Rc<RefCell<Trie>>, word: Rc<String>, pos: usize) {
            let mut inner = node.borrow_mut();

            if pos >= word.len() {
                inner.word = Some(word);
                return;
            }

            let ch = word.bytes().nth(pos).unwrap();
            let node_idx = (ch - b'a') as usize;
            assert!(node_idx < 26, "character out of range");

            let next_node = match &inner.children[node_idx] {
                Some(next_node) => next_node.clone(),
                None => {
                    let next_node: Rc<RefCell<Trie>> = Default::default();
                    inner.children[node_idx] = Some(next_node.clone());
                    next_node
                }
            };

            drop(inner);

            add_word(next_node, word, pos + 1);
        }

        for word in word_dict {
            add_word(root.clone(), Rc::new(word), 0);
        }

        let mut res = vec![];

        fn break_sentence(
            result: &mut Vec<String>,
            line: &str,
            root: Rc<RefCell<Trie>>,
            parts: &mut Vec<Rc<String>>,
        ) {
            // println!("Break: {} {:?}", line, parts);

            if line.len() == 0 {
                // terminal case: joining parts and put it to results
                if parts.len() > 0 {
                    let cap = parts.iter().map(|s| s.len()).sum::<usize>();
                    let mut sentence = String::with_capacity(cap + parts.len() - 1);

                    sentence.push_str(&parts[0]);

                    for word in parts.iter().skip(1) {
                        sentence.push(' ');
                        sentence.push_str(&word);
                    }

                    result.push(sentence);
                }

                return;
            }

            let mut idx = 0;
            let mut node = root.clone();

            loop {
                if let Some(word) = &node.borrow().word {
                    // When encounter a word, push it in parts and start a new search from root.
                    // then continue the current search by popping the word.
                    parts.push(word.clone());
                    break_sentence(result, &line[idx..], root.clone(), parts);
                    parts.pop();
                }

                if idx >= line.len() {
                    break;
                }

                let ch = line.bytes().nth(idx).unwrap();
                idx += 1;

                let node_idx = (ch - b'a') as usize;
                assert!(node_idx < 26);

                let next_node = match &node.borrow().children[node_idx] {
                    Some(next_node) => next_node.clone(),
                    None => break,
                };

                node = next_node;
            }
        }

        break_sentence(&mut res, &s, root.clone(), &mut vec![]);

        res
    }
}

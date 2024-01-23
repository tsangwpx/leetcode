mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 212

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct Node {
    word: Option<String>,
    children: [Option<Rc<RefCell<Node>>>; 26],
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn add_word(root: Rc<RefCell<Node>>, word: String) {
            let mut node = root;

            for ch in word.bytes() {
                let idx = (ch - b'a') as usize;
                let mut inner = node.borrow_mut();

                let next_node = match inner.children[idx] {
                    None => {
                        let next_node: Rc<RefCell<Node>> = Default::default();
                        inner.children[idx] = Some(next_node.clone());
                        next_node
                    }
                    Some(ref next_node) => next_node.clone(),
                };
                drop(inner);

                node = next_node;
            }

            node.borrow_mut().word = Some(word);
        }

        fn collect_words(
            board: &mut Vec<Vec<char>>,
            words: &mut Vec<String>,
            root: Rc<RefCell<Node>>,
            row: usize,
            col: usize,
        ) {
            let m = board.len();
            let n = board[row].len();
            let ch = board[row][col];

            if ch == '.' {
                return;
            }

            board[row][col] = '.';

            let idx = (ch as u8 - b'a') as usize;

            if let Some(this_node) = root.borrow().children[idx].clone() {
                {
                    let mut inner = this_node.borrow_mut();
                    if inner.word.is_some() {
                        words.push(std::mem::take(&mut inner.word).unwrap());
                    }
                }

                if row > 0 {
                    collect_words(board, words, this_node.clone(), row - 1, col);
                }

                if row + 1 < m {
                    collect_words(board, words, this_node.clone(), row + 1, col);
                }

                if col > 0 {
                    collect_words(board, words, this_node.clone(), row, col - 1);
                }

                if col + 1 < n {
                    collect_words(board, words, this_node.clone(), row, col + 1);
                }
            }

            board[row][col] = ch;
        }

        let mut board = board;
        let mut words = words;

        let root: Rc<RefCell<Node>> = Default::default();

        for word in words.drain(..) {
            add_word(root.clone(), word);
        }

        for row in 0..board.len() {
            for col in 0..board[row].len() {
                collect_words(&mut board, &mut words, root.clone(), row, col);
            }
        }

        words
    }
}

mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 3093
impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        #[derive(Debug)]
        struct Node {
            children: [Option<Box<Node>>; 26],
            min_len: usize,
            min_id: i32,
        }

        impl Node {
            fn default() -> Self {
                Node {
                    children: Default::default(),
                    min_len: usize::MAX,
                    min_id: i32::MAX,
                }
            }
        }

        fn insert(node: &mut Node, id: i32, word: String, pos: usize) {
            if node.min_len > word.len() {
                node.min_len = word.len();
                node.min_id = id;
            }

            if pos < word.len() {
                let ch = word.bytes().nth(pos).unwrap();
                let slot = ch as usize - b'a' as usize;
                let slot = &mut node.children[slot];
                if slot.is_none() {
                    *slot = Some(Box::new(Node::default()));
                }

                insert(slot.as_deref_mut().unwrap(), id, word, pos.wrapping_sub(1));
            }
        }

        fn search(node: &Node, query: String, pos: usize) -> i32 {
            let mut id = node.min_id;

            if pos < query.len() {
                let ch = query.bytes().nth(pos).unwrap();
                let slot = ch as usize - b'a' as usize;
                let slot = &node.children[slot];
                if let Some(ref node) = slot {
                    id = search(&node, query, pos.wrapping_sub(1));
                }
            }

            id
        }

        let mut root = Node::default();

        for (idx, word) in words_container.into_iter().enumerate() {
            let len = word.len();
            insert(&mut root, idx as i32, word, len - 1);
        }

        let mut res = vec![];

        for query in words_query.into_iter() {
            let len = query.len();
            let id = search(&root, query, len - 1);
            res.push(id);
        }

        res
    }
}

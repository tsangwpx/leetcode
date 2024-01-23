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
/**
 * This hash set version is slow because hashing the whole string is slow
 */
use std::collections::HashSet;

const WORD_LENGTH_MAX: usize = 25;

struct WordDictionary {
    tables: [HashSet<Vec<u8>>; WORD_LENGTH_MAX + 1],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            tables: Default::default(),
        }
    }

    fn add_word(&mut self, word: String) {
        self.tables[word.len()].insert(word.into_bytes());
    }

    fn search(&self, word: String) -> bool {
        let mut word = word.into_bytes();
        let table = &self.tables[word.len()];
        let dots = word
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(idx, ch)| if ch == b'.' { Some(idx) } else { None })
            .collect::<Vec<_>>();

        fn search_table(table: &HashSet<Vec<u8>>, word: &mut Vec<u8>, dots: &[usize]) -> bool {
            if dots.len() == 0 {
                return table.contains(word);
            }

            let idx = dots[0];

            for ch in b'a'..=b'z' {
                word[idx] = ch;
                if search_table(table, word, &dots[1..]) {
                    return true;
                }
            }

            false
        }

        search_table(&table, &mut word, &dots[..])
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
fn dummy() {}

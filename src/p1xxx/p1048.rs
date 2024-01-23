/*
You are given an array of words where each word consists of lowercase English letters.

wordA is a predecessor of wordB if and only if we can insert exactly one letter anywhere in wordA without changing the order of the other characters to make it equal to wordB.

    For example, "abc" is a predecessor of "abac", while "cba" is not a predecessor of "bcad".

A word chain is a sequence of words [word1, word2, ..., wordk] with k >= 1, where word1 is a predecessor of word2, word2 is a predecessor of word3, and so on. A single word is trivially a word chain with k == 1.

Return the length of the longest possible word chain with words chosen from the given list of words.



Example 1:

Input: words = ["a","b","ba","bca","bda","bdca"]
Output: 4
Explanation: One of the longest word chains is ["a","ba","bda","bdca"].

Example 2:

Input: words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
Output: 5
Explanation: All the words can be put in a word chain ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].

Example 3:

Input: words = ["abcd","dbqca"]
Output: 1
Explanation: The trivial word chain ["abcd"] is one of the longest word chains.
["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.
 */


use std::cmp::min;
use std::process::id;

struct Solution {}

impl Solution {
    fn is_predecessor(word1: &[u8], word2: &[u8]) -> bool {
        assert!(word1.len() < word2.len());
        match word1.iter().enumerate().position(|(idx, &ch)| ch != word2[idx]) {
            Some(pos) => word1[pos..].eq(&word2[pos + 1..]),
            None => true
        }
    }

    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        use std::cmp::max;
        use std::str::from_utf8;
        const WORDS_LENGTH_MAX: usize = 1000 + 1;

        let words = words.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
        assert!(words.len() <= WORDS_LENGTH_MAX);

        let mut global_chain_max = 1;
        let mut chain_lengths = [1u8; WORDS_LENGTH_MAX];
        let mut indices = [0usize; WORDS_LENGTH_MAX];

        let mut remaining = {
            let mut remaining = &mut indices[0..words.len()];
            for i in 0..remaining.len() {
                remaining[i] = i;
            }
            remaining.sort_unstable_by(|&a, &b| words[b].len().cmp(&words[a].len()));
            &*remaining
        };

        macro_rules! extract {
            {$main:ident, $part:ident, $word_len:expr} => {
                let ret = {
                    let word_len = $word_len;
                    let mid = remaining.partition_point(|&s| unsafe {words.get_unchecked(s).len()} >= word_len);
                    // remaining.split_at(mid)
                    unsafe { (remaining.get_unchecked(0..mid), remaining.get_unchecked(mid..)) }
                };
                $part = ret.0;
                $main = ret.1;
                drop(ret);
            };
        }

        let mut word_len = words[remaining[0]].len();
        let mut group2 = &remaining[0..0];
        let mut group1 = &remaining[0..0];

        extract! {remaining, group2, word_len}
        word_len -= 1;

        while remaining.len() > 0 {
            extract! {remaining, group1, word_len}

            for (idx1, word1) in group1.iter().map(|&s| (s, words[s])) {
                assert!(idx1 < chain_lengths.len());
                let mut word1_chain_max = chain_lengths[idx1];
                for (idx2, word2) in group2.iter().map(|&s| (s, words[s])) {
                    let yes = Self::is_predecessor(word1, word2);

                    if yes {
                        word1_chain_max = max(unsafe { *chain_lengths.get_unchecked(idx2) } + 1, word1_chain_max);
                    }

                    // println!("Compare {:?} {:?}: {}", from_utf8(word1), from_utf8(word2), yes);
                }

                chain_lengths[idx1] = word1_chain_max;
                global_chain_max = max(word1_chain_max, global_chain_max);
            }

            word_len -= 1;
            group2 = group1;
        }

        global_chain_max as i32
    }


    pub fn longest_str_chain2(words: Vec<String>) -> i32 {
        use std::cmp::max;
        use std::str::from_utf8;
        const WORDS_LENGTH_MAX: usize = 1000;

        let words = words.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
        let words_by_len = {
            let mut words_by_len = vec![vec![0usize; 0]; WORDS_LENGTH_MAX + 1];
            let mut word_len_max = 0;
            for (i, &word) in words.iter().enumerate() {
                words_by_len[word.len()].push(i);
                word_len_max = max(word_len_max, word.len());
            }
            words_by_len.truncate(word_len_max + 1);
            words_by_len
        };

        let mut chain_lengths = vec![1; words.len()];
        let mut global_chain_max = 1;


        for word_len in (1..words_by_len.len()).rev() {
            let group2 = &words_by_len[word_len];
            let group1 = &words_by_len[word_len - 1];

            for (idx1, word1) in group1.iter().map(|&s| (s, words[s])) {
                let mut word1_chain_max = chain_lengths[idx1];
                for (idx2, word2) in group2.iter().map(|&s| (s, words[s])) {
                    let yes = Self::is_predecessor(word1, word2);

                    if yes {
                        word1_chain_max = max(chain_lengths[idx2] + 1, word1_chain_max);
                    }

                    println!("Compare {:?} {:?}: {}", from_utf8(word1), from_utf8(word2), yes);
                }

                chain_lengths[idx1] = word1_chain_max;
                global_chain_max = max(word1_chain_max, global_chain_max);
            }
        }

        global_chain_max
    }
}

fn main() {
    let soln = Solution {};
}
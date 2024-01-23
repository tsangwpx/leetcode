use std::process::id;

#[derive(Debug)]
struct UniqueWord {
    word: String,
    reversed: String,
    index: usize,
}

impl UniqueWord {
    fn new(word: String, index: usize) -> UniqueWord {
        let mut vec = word.clone().into_bytes();
        vec.reverse();
        // SAFETY: Only ASCII characters in word
        let reversed = unsafe { String::from_utf8_unchecked(vec) };
        Self { word, reversed, index }
    }

    fn key(wi: &Self) -> &String {
        &wi.word
    }

    fn reversed_key(wi: &Self) -> &String {
        &wi.reversed
    }
}


struct WordFilter {
    // Unique words
    unique_words: Vec<UniqueWord>,
    // Index of unique words sorted by `UniqueWord.word`
    prefix_indices: Vec<usize>,
    // Index of unique words sorted by `UniqueWord.reversed`
    suffix_indices: Vec<usize>,
}


impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        use std::collections::HashMap;

        // Keep repeated words with larger index
        let word_index_table = words
            .into_iter()
            .enumerate()
            .map(|(idx, word)| (word, idx))
            .collect::<HashMap<String, usize>>();

        let unique_words = word_index_table
            .into_iter()
            .map(|s| UniqueWord::new(s.0, s.1))
            .collect::<Vec<UniqueWord>>();

        let mut prefix_indices = (0..unique_words.len()).collect::<Vec<usize>>();
        let mut suffix_indices = prefix_indices.clone();
        // prefix_indices.sort_by(|&a, &b| word_indices[a].word.cmp(&word_indices[b].word));

        prefix_indices.sort_by_key(|&s| &unique_words[s].word);
        suffix_indices.sort_by_key(|&s| &unique_words[s].reversed);

        // println!("unique: {:?}", unique_words);
        // println!("prefix: {:?}", prefix_indices);
        // println!("suffix: {:?}", suffix_indices);

        Self { unique_words, prefix_indices, suffix_indices }
    }

    fn prefix_range(&self, needle: &String) -> std::ops::Range<usize> {
        self.search_range(&self.prefix_indices, UniqueWord::key, needle)
    }

    fn suffix_range(&self, needle: &String) -> std::ops::Range<usize> {
        self.search_range(&self.suffix_indices, UniqueWord::reversed_key, needle)
    }

    /// Return a range of an index table where the key of the corresponding UniqueWord entry starting with needle.
    fn search_range<F>(
        &self,
        table: &Vec<usize>,
        key: F,
        needle: &String,
    ) -> std::ops::Range<usize> where
        F: for<'a> Fn(&'a UniqueWord) -> &'a String {
        assert_eq!(self.unique_words.len(), table.len());
        use std::cmp::{max, min, Ordering};

        let start = table
            .binary_search_by(|&s| key(&self.unique_words[s]).cmp(needle))
            .unwrap_or_else(|s| s);

        let mut stop = table
            .binary_search_by(|&s| {
                let left = key(&self.unique_words[s]).as_bytes();
                let right = needle.as_bytes();
                let len = min(left.len(), right.len());
                left[..len].cmp(&right[..len]).then(Ordering::Less)
            })
            .unwrap_or_else(|s| s);

        start..stop
    }

    pub fn f(&self, prefix: String, suffix: String) -> i32 {
        // println!("Search {:?} {:?}", prefix, suffix);
        use std::collections::HashSet;
        use std::cmp::{max, min};

        let prefix_range = self.prefix_range(&prefix);
        // println!("Prefix range {:?}", prefix_range);
        if prefix_range.len() == 0 {
            return -1;
        }

        let mut suffix_vec = suffix.into_bytes();
        suffix_vec.reverse();
        let suffix = unsafe { String::from_utf8_unchecked(suffix_vec) };

        let suffix_range = self.suffix_range(&suffix);
        // println!("Suffix range {:?}", suffix_range);
        if suffix_range.len() == 0 {
            return -1;
        }

        // build the candidate set with smaller range
        type Spec<'a> = (&'a Vec<usize>, &'a std::ops::Range<usize>, &'a String, for<'r> fn(&'r UniqueWord) -> &'r String);
        let mut can_spec: Spec = (&self.prefix_indices, &prefix_range, &prefix, UniqueWord::key);
        let mut fin_spec: Spec = (&self.suffix_indices, &suffix_range, &suffix, UniqueWord::reversed_key);

        if prefix_range.len() > suffix_range.len() {
            std::mem::swap(&mut can_spec, &mut fin_spec);
        }

        let (can_indices, can_range, can_needle, can_key) = can_spec;
        assert_eq!(self.unique_words.len(), can_indices.len());
        assert!(can_range.end <= can_indices.len());

        let mut candidates = HashSet::<usize>::with_capacity(can_range.len());
        for i in can_range.clone() {
            let uw_idx = can_indices[i];
            let uw = &self.unique_words[uw_idx];
            if can_key(uw).starts_with(can_needle) {
                candidates.insert(uw_idx);
            }
        }

        let mut result = -1i32;
        let (fin_indices, fin_range, fin_needle, fin_key) = fin_spec;
        for i in fin_range.clone() {
            let uw_idx = fin_indices[i];
            let uw = &self.unique_words[uw_idx];
            if candidates.contains(&uw_idx) && fin_key(uw).starts_with(fin_needle) {
                result = max(result, uw.index as i32);
            }
        }

        result
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

fn main() {
    let wf = WordFilter::new(vec!["apple".to_owned()]);
    wf.f("a".to_string(), "e".to_string());
}
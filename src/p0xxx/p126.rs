// Problem  126
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        const SHIFT: u32 = 5;
        const MASK: u32 = 2u32.pow(SHIFT) - 1;

        #[inline]
        fn get_wildcards(word: &str) -> Vec<u32> {
            // compute variant keys of a given word
            let mut base = 0;
            for i in 0..word.len() {
                base <<= SHIFT;
                base |= (word.bytes().nth(i).unwrap() - b'a') as u32;
            }

            let base = base;

            let mut variants = vec![0u32; word.len()];

            for i in 0..variants.len() {
                let shift = SHIFT * i as u32;
                let mask = MASK << shift;
                variants[i] = base | mask;
            }

            variants
        }

        let mut begin_idx = None;
        let mut end_idx = None;

        word_list.iter().enumerate().for_each(|(pos, word)| {
            if word == &begin_word {
                begin_idx = Some(pos);
            }
            if word == &end_word {
                end_idx = Some(pos);
            }
        });

        let Some(end_idx) = end_idx else {
            return vec![];
        };

        let begin_idx = begin_idx.unwrap_or_else(|| {
            let pos = word_list.len();
            word_list.push(begin_word);
            pos
        });

        let word_list = word_list;

        // map wildcard word to a list of original words
        let mut table = HashMap::<u32, Vec<u16>>::new();

        // build the wildcard word table
        for (idx, word) in word_list.iter().enumerate() {
            let variants = get_wildcards(&word);
            for key in variants.iter().copied() {
                table.entry(key).or_insert_with(|| vec![]).push(idx as u16);
            }
        }

        // record the depth of each word. Starts from 0.
        let mut dists = vec![u16::MAX; word_list.len()];
        dists[begin_idx] = 0;

        // the index of the end word in output. or MAX if not found.
        let mut max_depth = u16::MAX;
        let mut queue = VecDeque::with_capacity(word_list.len());
        queue.push_back((0, begin_idx));

        let mut backtrace = vec![vec![]; word_list.len()];

        while let Some((depth, word_idx)) = queue.pop_front() {
            if depth > max_depth {
                // beyond the search.
                break;
            }

            let new_depth = depth + 1;
            let variants = get_wildcards(&word_list[word_idx]);

            for key in variants.iter().copied() {
                for &neighbour_idx in table.get(&key).unwrap() {
                    let neighbour_idx = neighbour_idx as usize;

                    if neighbour_idx == word_idx {
                        // skip same word
                        continue;
                    }

                    if dists[neighbour_idx] == u16::MAX {
                        // this neighbor is new.

                        if neighbour_idx == end_idx {
                            // this is the end.
                            max_depth = new_depth;
                        }

                        // assign new depth and schedule it to fill its backtrace later.
                        dists[neighbour_idx] = new_depth;
                        queue.push_back((new_depth, neighbour_idx));
                    } else if dists[neighbour_idx] + 1 == depth {
                        // this neighbour is previous word in backtrace.
                        backtrace[word_idx].push(neighbour_idx as u16);
                    }
                }
            }
        }

        if max_depth == u16::MAX {
            return vec![];
        }

        fn bfs(
            res: &mut Vec<Vec<String>>,
            word_list: &Vec<String>,
            backtrace: &Vec<Vec<u16>>,
            seq: &mut Vec<u16>,
            depth: u16,
        ) {
            if depth == 0 {
                // this is the first index. End of backtrace. Output the result.
                res.push(
                    seq.iter()
                        .map(|&idx| word_list[idx as usize].clone())
                        .collect::<Vec<_>>(),
                );
                return;
            }

            let item_idx = seq[depth as usize];

            for prev_idx in backtrace[item_idx as usize].iter().copied() {
                seq[depth as usize - 1] = prev_idx;
                bfs(res, word_list, backtrace, seq, depth - 1);
            }
        }

        // `backtrace` is built from begin_word.
        // We may build the result when traverse in reverse.

        let mut res = vec![];
        let mut seq = vec![u16::MAX; max_depth as usize + 1];
        seq[max_depth as usize] = end_idx as u16;

        bfs(&mut res, &word_list, &backtrace, &mut seq, max_depth);

        res
    }
}

// Problem 1625
impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        // brute-force?

        use std::collections::HashMap;

        type HashSet = HashMap<Vec<u8>, ()>;

        #[inline]
        fn enqueue(pending: &mut Vec<Vec<u8>>, seen: &mut HashSet, item: Vec<u8>) {
            seen.entry(item).or_insert_with_key(|key| {
                pending.push(key.clone());
                // println!("Enqueue {:?}", key)
            });
        }

        let mut seen = HashSet::with_capacity(s.len() * 10 * 10);
        let mut pending = vec![];

        enqueue(&mut pending, &mut seen, s.into_bytes());

        while let Some(seq) = pending.pop() {
            let seq2 = seq
                .iter()
                .copied()
                .enumerate()
                .map(|(idx, ch)| {
                    if idx % 2 == 0 {
                        ch
                    } else {
                        (((ch - b'0') + a as u8) % 10) + b'0'
                    }
                })
                .collect::<Vec<u8>>();
            enqueue(&mut pending, &mut seen, seq2);

            let skip = seq.len() - b as usize;

            let seq3 = seq
                .iter()
                .copied()
                .skip(skip)
                .chain(seq.iter().copied().take(skip))
                .collect::<Vec<u8>>();

            enqueue(&mut pending, &mut seen, seq3);
        }

        String::from_utf8(seen.keys().min().cloned().unwrap()).unwrap()
    }
}

// Problem 756
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        // There are 6 letters. They can be numbered using 3 bits, from 0 to 5 (inclusive).
        // the value 7 (0b111 in binary) is used to represent absence of value.
        // To store them all, 3 * 6 = 18 bits are required and it can be fitted into a u32 type.

        use std::collections::HashMap;

        #[inline]
        fn letter_id(ch: u8) -> u8 {
            ch - b'A'
        }

        const LETTER_SHIFT: u32 = 3;
        const LETTER_MASK: u32 = (1 << LETTER_SHIFT) - 1;
        assert!(LETTER_SHIFT <= 3);

        let mut tmp = [0u8; 6];

        let bottom = {
            bottom
                .bytes()
                .enumerate()
                .for_each(|(idx, ch)| tmp[idx] = letter_id(ch));
            &tmp[0..bottom.len()]
        };

        let mut transitions = [[u32::MAX; 6]; 6];

        let mut memo = HashMap::<u32, bool>::new();

        for triplet in allowed.iter() {
            let &[left, right, above] = triplet.as_bytes() else {
                panic!("bad format");
            };

            let left = letter_id(left);
            let right = letter_id(right);
            let above = letter_id(above);

            let value = &mut transitions[left as usize][right as usize];
            *value = ((*value) << LETTER_SHIFT) | ((above as u32) & LETTER_MASK);
        }

        #[inline]
        fn row2id(row: &[u8]) -> u32 {
            row.iter()
                .take(6)
                .fold(u32::MAX, |acc, &ch| (acc << LETTER_SHIFT) | (ch as u32))
        }

        fn dfs(memo: &mut HashMap<u32, bool>, transitions: &[[u32; 6]; 6], bottom: &[u8]) -> bool {
            if bottom.len() == 1 {
                return true;
            }

            let bottom_id = row2id(&bottom);

            if let Some(res) = memo.get(&bottom_id).copied() {
                return res;
            }

            let mut row = [0u8; 6];

            let res = dfs2(
                memo,
                transitions,
                bottom,
                &mut row[0..bottom.len().saturating_sub(1)],
                0,
            );
            // println!("{:?} {} {}", bottom, bottom_id, res);
            memo.insert(bottom_id, res);
            res
        }

        fn dfs2(
            memo: &mut HashMap<u32, bool>,
            transitions: &[[u32; 6]; 6],
            bottom: &[u8],
            row: &mut [u8],
            idx: usize,
        ) -> bool {
            if idx >= row.len() || idx + 1 >= bottom.len() {
                dfs(memo, transitions, &row)
            } else {
                let left = bottom[idx];
                let right = bottom[idx + 1];
                let mut targets = transitions[left as usize][right as usize];

                while (targets & LETTER_MASK) != LETTER_MASK {
                    row[idx] = (targets & LETTER_MASK) as u8;
                    // note that the msb is no longer one, which may cause infinite loop if LETTER_SHIFT >= 4.
                    targets >>= LETTER_SHIFT;

                    if dfs2(memo, transitions, bottom, row, idx + 1) {
                        return true;
                    }
                }

                false
            }
        }

        dfs(&mut &mut memo, &transitions, &bottom)
    }
}

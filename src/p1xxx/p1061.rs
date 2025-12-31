// Problem 1061
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        // union-find

        let mut table = [0u8; 26];
        for i in 0..26 {
            table[i] = i as u8;
        }

        #[inline]
        fn find(table: &mut [u8; 26], idx: u8) -> u8 {
            let mut parent = table[idx as usize];

            if parent != table[parent as usize] {
                parent = find(table, parent);
                table[idx as usize] = parent;
            }

            parent
        }

        #[inline]
        fn union(table: &mut [u8; 26], idx: u8, idx2: u8) -> u8 {
            let idx = find(table, idx);
            let idx2 = find(table, idx2);
            let (idx, idx2) = (idx.min(idx2), idx.max(idx2));
            table[idx2 as usize] = idx;
            idx
        }

        for (ch1, ch2) in s1.bytes().zip(s2.bytes()) {
            union(&mut table, ch1 - b'a', ch2 - b'a');
        }

        base_str
            .chars()
            .map(|ch| {
                let ch = find(&mut table, ch as u8 - b'a') + b'a';
                ch as char
            })
            .collect::<String>()
    }
}

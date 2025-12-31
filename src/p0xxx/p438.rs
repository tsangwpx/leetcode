use core::num;

// Problem 438
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        assert!(s.len() >= 1);
        assert!(p.len() >= 1);

        let mut res = vec![];

        if s.len() < p.len() {
            return res;
        }

        #[inline]
        fn ord(ch: u8) -> usize {
            (ch - b'a') as usize
        }

        #[inline]
        fn ord_nth(str: &str, idx: usize) -> usize {
            ord(str.bytes().nth(idx).unwrap())
        }

        let mut target = [0u16; 26];
        let mut counter = [0u16; 26];

        for ch in p.bytes() {
            target[ord(ch)] += 1;
        }

        let target = target;

        // res.reserve(s.len() - p.len() + 1);

        for idx in 0..(p.len() - 1) {
            counter[ord_nth(&s, idx)] += 1;
        }

        for idx in (p.len() - 1)..s.len() {
            let stop = idx + 1;
            let start = stop - p.len();

            let offset = ord_nth(&s, idx);
            counter[offset] += 1;

            if target[offset] == counter[offset] && target == counter {
                res.push(start as i32);
            }
            counter[ord_nth(&s, start)] -= 1;
        }

        res
    }
}

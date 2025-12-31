// Problem 1544
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut s = s.into_bytes();
        let mut read = 0;
        let mut write = 0;

        while read < s.len() && write < s.len() {
            let ch = s[read];
            read += 1;

            let good = if write == 0 {
                true
            } else {
                let last = s[write - 1];

                (ch.to_ascii_lowercase() != last.to_ascii_lowercase())
                    || (ch.is_ascii_lowercase() == last.is_ascii_lowercase())
            };

            if good {
                s[write] = ch;
                write += 1;
            } else {
                write -= 1;
            }
        }
        s.truncate(write);
        String::from_utf8(s).unwrap()
    }

    pub fn make_good2(s: String) -> String {
        let s = s.into_bytes();
        let mut res = Vec::with_capacity(s.len());
        for ch in s.iter().copied() {
            let Some(last) = res.last().copied() else {
                res.push(ch);
                continue;
            };

            if (ch.to_ascii_lowercase() == last.to_ascii_lowercase())
                && (ch.is_ascii_lowercase() != last.is_ascii_lowercase())
            {
                res.pop();
            } else {
                res.push(ch);
            }
        }

        String::from_utf8(res).unwrap()
    }
}

// Problem 3039
impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        #[derive(Default, Clone, Copy)]
        struct Record {
            count: i32,
            pos: u32,
        }

        let mut table = [Record::default(); 26];

        for (pos, ch) in s.bytes().enumerate() {
            let idx = (ch - b'a') as usize;
            table[idx].count += 1;
            table[idx].pos = pos as u32;
        }

        table.sort_unstable_by_key(|s| (-s.count, s.pos));

        let max_count = table[0].count;
        let mut res = String::new();

        for record in table {
            if record.count == max_count {
                res.push(s.bytes().nth(record.pos as usize).unwrap() as char);
            } else {
                break;
            }
        }

        res
    }
}

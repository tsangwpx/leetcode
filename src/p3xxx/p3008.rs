// Problem 3008
impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        if s.len() < a.len() || s.len() < b.len() {
            return vec![];
        }

        fn build_table(pattern: &str) -> Vec<usize> {
            let mut table = vec![0usize; pattern.len()];
            let mut idx = 1;
            let mut len = 0usize; // length of the proper prefix which is also suffix

            while idx < pattern.len() && len < pattern.len() {
                if pattern.bytes().nth(idx) == pattern.bytes().nth(len) {
                    // we may increase the prefix len by 1
                    // pattern[idx - len..idx + 1]== pattern[...len + 1]
                    len += 1;
                    table[idx] = len;
                    idx += 1;
                } else if len > 0 {
                    // prefix cannot be of length = len + 1
                    // we look for length = (len - 1) + 1
                    // we check previous result in the table to obtain the longest possible len.
                    len = table[len - 1];
                } else {
                    // only empty prefix / suffix is possible
                    table[idx] = 0;
                    idx += 1;
                }
            }

            table
        }

        fn find_occurences(string: &str, pattern: &str) -> Vec<usize> {
            let table = build_table(&pattern);
            // println!("table: {:?}", table);
            assert!(table.len() == pattern.len());

            let stop = match string.len() {
                0 => 0,
                s => s + 1 - pattern.len(),
            };

            let mut indexes = vec![];
            let mut idx = 0;
            let mut len = 0;

            while idx < stop {
                // println!(
                //     "{} {}: {} {}",
                //     idx,
                //     len,
                //     string.chars().nth(idx + len).unwrap(),
                //     pattern.chars().nth(len).unwrap()
                // );

                // update idx or not
                let mut stale = false;

                if pattern.bytes().nth(len).unwrap() == string.bytes().nth(idx + len).unwrap() {
                    len += 1;

                    if len >= pattern.len() {
                        indexes.push(idx);
                        stale = true;
                    }
                } else {
                    stale = true;
                }

                if stale {
                    if len > 0 {
                        idx += len - table[len - 1];
                        len = table[len - 1];
                    } else {
                        idx += 1;
                    }
                }
            }

            // println!("{:?}", indexes);

            indexes
        }

        let k = k as usize;
        let a_indexes = find_occurences(&s, &a);
        let b_indexes = find_occurences(&s, &b);

        let mut res = vec![];
        let mut front = 0;

        'outer: for i in a_indexes.iter().cloned() {
            while front < b_indexes.len() {
                if i.abs_diff(b_indexes[front]) <= k {
                    res.push(i as i32);
                    break;
                }
                if b_indexes[front] > i + k {
                    break;
                }

                front += 1;

                if front >= b_indexes.len() {
                    break 'outer;
                }
            }
        }

        res
    }
}

// Problem 2014
impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        use std::collections::VecDeque;

        fn is_subseq(long: &[u8], short: &[u8], k: usize) -> bool {
            let mut index = 0;
            let mut repeat = 0;

            for ch in long.iter().copied() {
                if index < short.len() && ch == short[index] {
                    index += 1;

                    if index >= short.len() {
                        index = 0;
                        repeat += 1;

                        if repeat >= k {
                            return true;
                        }
                    }
                }
            }

            false
        }

        let s = s.into_bytes();
        let k = k as usize;

        let counter = s.iter().copied().fold([0; 26], |mut acc, ch| {
            acc[(ch - b'a') as usize] += 1;
            acc
        });
        let chars = counter
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut v, (idx, &count)| {
                let count = count as usize;
                if count >= k {
                    v.push(idx as u8 + b'a');
                }
                v
            });

        println!(
            "chars {:?}",
            chars.iter().map(|&s| s as char).collect::<String>()
        );

        let mut deque = VecDeque::new();
        let mut work = vec![];
        let mut res = vec![];

        deque.push_back(Vec::new());

        while let Some(base) = deque.pop_front() {
            for ch in chars.iter().copied() {
                work.clone_from(&base);
                work.push(ch);
                println!("{:?}", work);
                if is_subseq(&s, &work, k) {
                    work.clone_into(&mut res);
                    deque.push_back(work.clone());
                }
            }
        }

        String::from_utf8(res).unwrap()
    }
}

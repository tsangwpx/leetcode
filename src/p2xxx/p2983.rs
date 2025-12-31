// Problem 2983
impl Solution {
    pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        #[inline]
        fn reflect(i: usize, n: usize) -> usize {
            n - 1 - i
        }

        const FREQ_LEN: usize = 26; // multiple of 4
        type CharFreq = [i32; FREQ_LEN];

        #[inline]
        fn sub(mut a: CharFreq, b: CharFreq) -> CharFreq {
            for i in 0..a.len() {
                a[i] -= b[i];
            }

            a
        }

        let n = s.len();
        let h = n / 2;

        let mut same_lengths = vec![0usize; h];
        {
            let mut count = 0;
            for i in 0..h {
                if s.bytes().nth(i).unwrap() == s.bytes().nth(reflect(i, n)).unwrap() {
                    count += 1;
                } else {
                    count = 0;
                }

                same_lengths[i] = count;
            }
        }

        #[inline]
        fn is_same(same_lengths: &Vec<usize>, start: usize, stop: usize) -> bool {
            // where s[start..stop] is same, stop <= h
            if start >= stop {
                return true;
            }

            let len = stop - start;
            return same_lengths[stop - 1] >= len;
        }

        let mut upper_freq = vec![CharFreq::default(); h + 1];
        let mut lower_freq = vec![CharFreq::default(); h + 1];

        {
            let mut tmp = CharFreq::default();
            for i in 0..h {
                let idx = (s.bytes().nth(i).unwrap() - b'a') as usize;
                tmp[idx] += 1;
                upper_freq[i + 1] = tmp;
            }
        }

        {
            let mut tmp = CharFreq::default();
            for i in 0..h {
                let idx = (s.bytes().nth(reflect(i, n)).unwrap() - b'a') as usize;
                tmp[idx] += 1;
                lower_freq[i + 1] = tmp;
            }
        }

        let mut memo = std::collections::HashMap::<&Vec<i32>, bool>::new();

        queries
            .iter()
            .map(|query| {
                if let Some(&result) = memo.get(query) {
                    return result;
                }
                assert!(query.len() == 4);

                let a = query[0] as usize;
                let b = query[1] as usize;
                let c = reflect(query[2] as usize, n);
                let d = reflect(query[3] as usize, n);

                if !is_same(&same_lengths, 0, a.min(d)) {
                    return false;
                }

                if !is_same(&same_lengths, b.max(c) + 1, h) {
                    return false;
                }

                let mut left_freq = sub(upper_freq[b + 1], upper_freq[a]);

                let mut right_freq = sub(lower_freq[c + 1], lower_freq[d]);

                if b < d || c < a {
                    // no intersection
                    //  A _ _ B _ _ Y Y Y Y Y
                    //  X X X X _ _ D _ _ _ C

                    // Make sure the middle part is same
                    if !is_same(&same_lengths, b + 1, d) {
                        return false;
                    }

                    if !is_same(&same_lengths, c + 1, a) {
                        return false;
                    }

                    // Remove XXX from A...B
                    let xxx = sub(lower_freq[b + 1], lower_freq[a]);
                    left_freq = sub(left_freq, xxx);

                    // remove YYY from C...D
                    let yyy = sub(upper_freq[c + 1], upper_freq[d]);
                    right_freq = sub(right_freq, yyy);

                    // println!("QuerN: {:?}\n{:?}\n{:?}", query, left_freq, right_freq);

                    return left_freq == CharFreq::default() && right_freq == CharFreq::default();
                }

                //  A _ _ _ _ _ B Y Y _
                //  X X X D'_ _ _ _ C'_

                //  _ X X A _ _ _ _ _ B _
                //  _ D'_ _ _ _ C Y Y Y _

                if a < d {
                    // Remove XXX from A ... B
                    let xxx = sub(lower_freq[d], lower_freq[a]);
                    left_freq = sub(left_freq, xxx);
                } else if a > d {
                    // Remove XXX from C ... D
                    let xxx = sub(upper_freq[a], upper_freq[d]);
                    right_freq = sub(right_freq, xxx);
                }

                if b < c {
                    // Remove YYY from C ... D
                    let yyy = sub(upper_freq[c + 1], upper_freq[b + 1]);
                    right_freq = sub(right_freq, yyy);
                } else if b > c {
                    // Remove YYYY from A ... B
                    let yyy = sub(lower_freq[b + 1], lower_freq[c + 1]);
                    left_freq = sub(left_freq, yyy);
                }

                // println!("QuerY: {:?}\n{:?}\n{:?}", query, left_freq, right_freq);
                // what is remained is the intersection
                // they should match in distribution and all nonnegative

                let result = left_freq
                    .iter()
                    .copied()
                    .zip(right_freq.iter().copied())
                    .all(|(p, q)| p >= 0 && p == q);
                memo.insert(query, result);
                result
            })
            .collect::<Vec<bool>>()
    }

    pub fn can_make_palindrome_queries2(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        #[inline]
        fn reflect(i: usize, n: usize) -> usize {
            n - 1 - i
        }

        #[inline]
        fn sub(mut a: [i32; 26], b: [i32; 26]) -> [i32; 26] {
            for i in 0..a.len() {
                a[i] -= b[i];
            }

            a
        }

        #[inline]
        fn add(mut a: [i32; 26], b: [i32; 26]) -> [i32; 26] {
            for i in 0..a.len() {
                a[i] += b[i];
            }

            a
        }

        let n = s.len();
        let h = n / 2;

        let mut same_lengths = vec![0usize; h];
        {
            let mut count = 0;
            for i in 0..h {
                if s.bytes().nth(i).unwrap() == s.bytes().nth(reflect(i, n)).unwrap() {
                    count += 1;
                } else {
                    count = 0;
                }

                same_lengths[i] = count;
            }
        }

        #[inline]
        fn is_same(same_lengths: &Vec<usize>, start: usize, stop: usize) -> bool {
            if start >= stop {
                return true;
            }

            let len = stop - start;
            return same_lengths[stop - 1] >= len;
        }

        let mut distributions = vec![[0i32; 26]; n];
        {
            let mut tmp = [0i32; 26];
            for i in 0..h {
                let idx = (s.bytes().nth(i).unwrap() - b'a') as usize;
                tmp[idx] += 1;
                distributions[i] = tmp;
            }

            let mut tmp = [0i32; 26];
            for i in (h..n).rev() {
                let idx = (s.bytes().nth(i).unwrap() - b'a') as usize;
                tmp[idx] += 1;
                distributions[i] = tmp;
            }
        }

        let mut memo = std::collections::HashMap::<&Vec<i32>, bool>::new();

        queries
            .iter()
            .map(|query| {
                assert!(query.len() == 4);

                if let Some(&result) = memo.get(query) {
                    return result;
                }

                let a = query[0] as usize;
                let b = query[1] as usize;
                let c = query[2] as usize;
                let d = query[3] as usize;
                let a_refl = reflect(a, n);
                let b_refl = reflect(b, n);
                let c_refl = reflect(c, n);
                let d_refl = reflect(d, n);

                // println!(
                //     "{} {} {} {}: {} {} {} {}",
                //     a, b, c, d, a_refl, b_refl, c_refl, d_refl
                // );

                if !is_same(&same_lengths, 0, a.min(d_refl)) {
                    return false;
                }

                if !is_same(&same_lengths, b.max(c_refl) + 1, h) {
                    return false;
                }

                let mut left_freq = distributions[b];
                if a > 0 {
                    left_freq = sub(left_freq, distributions[a - 1]);
                }

                let mut right_freq = distributions[c];
                if d + 1 < n {
                    right_freq = sub(right_freq, distributions[d + 1]);
                }

                if b < d_refl || c_refl < a {
                    // no intersection
                    //  A _ _ B _ _ Y Y Y Y Y
                    //  X X X X _ _ D'_ _ _ C'

                    if !is_same(&same_lengths, b + 1, d_refl) {
                        return false;
                    }

                    if !is_same(&same_lengths, c_refl + 1, a) {
                        return false;
                    }

                    // Remove XXX from A...B
                    let mut xxx = distributions[b_refl];
                    if a_refl + 1 < n {
                        xxx = sub(xxx, distributions[a_refl + 1]);
                    }
                    left_freq = sub(left_freq, xxx);

                    // remove YYY from C...D
                    let mut yyy = distributions[c_refl];
                    if d_refl > 0 {
                        yyy = sub(yyy, distributions[d_refl - 1]);
                    }
                    right_freq = sub(right_freq, yyy);

                    // println!("QuerN: {:?}\n{:?}\n{:?}", query, left_freq, right_freq);

                    return left_freq == [0i32; 26] && right_freq == [0i32; 26];
                }

                //  A _ _ _ _ _ B Y Y _
                //  X X X D'_ _ _ _ C'_

                //  _ X X A _ _ _ _ _ B _
                //  _ D'_ _ _ _ C Y Y Y _

                if a < d_refl {
                    // Remove XXX from A ... B
                    let mut xxx = distributions[d + 1];

                    if a_refl + 1 < n {
                        xxx = sub(xxx, distributions[a_refl + 1]);
                    }
                    left_freq = sub(left_freq, xxx);
                } else if a > d_refl {
                    // Remove XXX from C ... D
                    let mut xxx = distributions[a - 1];
                    if d_refl > 0 {
                        xxx = sub(xxx, distributions[d_refl - 1]);
                    }
                    right_freq = sub(right_freq, xxx);
                }

                if b < c_refl {
                    // Remove YYY from C ... D
                    let yyy = sub(distributions[c_refl], distributions[b]);
                    right_freq = sub(right_freq, yyy);
                } else if b > c_refl {
                    // Remove YYYY from A ... B
                    let yyy = sub(distributions[b_refl], distributions[c]);
                    left_freq = sub(left_freq, yyy);
                }

                // println!("QuerY: {:?}\n{:?}\n{:?}", query, left_freq, right_freq);
                // what is remained is the intersection
                // they should match in distribution and all nonnegative

                let result = left_freq
                    .iter()
                    .copied()
                    .zip(right_freq.iter().copied())
                    .all(|(p, q)| p >= 0 && p == q);
                memo.insert(query, result);
                result
            })
            .collect::<Vec<bool>>()
    }
}

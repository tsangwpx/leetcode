// Problem 2531
impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        #[inline]
        fn letter_frequency(word: String) -> ([i8; 26], usize) {
            let mut freq = [0i8; 26];
            word.bytes().for_each(|ch| {
                let idx = (ch - b'a') as usize;

                if idx < freq.len() && freq[idx] < 64 {
                    freq[idx] += 1;
                }
            });

            let dist = freq.iter().filter(|&&s| s > 0).count();

            (freq, dist)
        }

        let (freq1, dist1) = letter_frequency(word1);
        let (freq2, dist2) = letter_frequency(word2);

        // println!("{}: {:?}", dist1, freq1);
        // println!("{}: {:?}", dist2, freq2);

        for i in 0..26 {
            for j in 0..26 {
                if freq1[i] == 0 || freq2[j] == 0 {
                    continue;
                }

                if i == j {
                    if dist1 == dist2 {
                        return true;
                    }
                } else {
                    let mut new_dist1 = dist1;
                    let mut new_dist2 = dist2;

                    if freq1[i] == 1 {
                        new_dist1 -= 1;
                    }
                    if freq1[j] == 0 {
                        new_dist1 += 1;
                    }

                    if freq2[j] == 1 {
                        new_dist2 -= 1;
                    }
                    if freq2[i] == 0 {
                        new_dist2 += 1;
                    }
                    // println!("{} {}: {} {}", i, j, new_dist1, new_dist2);
                    if new_dist1 == new_dist2 {
                        return true;
                    }
                }
            }
        }

        false
    }
}

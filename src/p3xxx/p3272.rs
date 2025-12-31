// Problem 3272
impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        use std::collections::HashSet;

        type PalindromeHash = [u8; 10];

        fn to_i64(buf: &Vec<u8>) -> i64 {
            buf.iter()
                .rev()
                .copied()
                .fold(0i64, |acc, digit| acc * 10 + digit as i64)
        }

        fn check_and_add(hashes: &mut HashSet<PalindromeHash>, buf: &Vec<u8>, k: i32) {
            if buf.is_empty() || buf[0] == 0 {
                return;
            }

            let number = to_i64(buf);
            let k = k as i64;

            // micro optimization?
            let k_divisible = match k {
                1 => number % k == 0,
                2 => number % k == 0,
                3 => number % k == 0,
                4 => number % k == 0,
                5 => number % k == 0,
                6 => number % k == 0,
                7 => number % k == 0,
                8 => number % k == 0,
                9 => number % k == 0,
                _ => false,
            };

            if !k_divisible {
                return;
            }

            let mut key = PalindromeHash::default();

            for item in buf.iter().copied() {
                let item = item as usize;

                if item < key.len() {
                    key[item] += 1;
                }
            }

            // println!("Add {:?} as {:?}", buf, key);

            hashes.insert(key);
        }

        fn search_palindromes(
            hashes: &mut HashSet<PalindromeHash>,
            buf: &mut Vec<u8>,
            left: usize,
            k: i32,
        ) {
            let right = buf.len() - 1 - left;

            // right may be overflow, so we check left >= buf.len()
            if left >= buf.len() || right < left {
                check_and_add(hashes, buf, k);
                return;
            }

            if left == right {
                for digit in 0..=9 {
                    buf[left] = digit;
                    search_palindromes(hashes, buf, left + 1, k);
                }
            } else {
                for digit in 0..=9 {
                    buf[left] = digit;
                    buf[right] = digit;
                    search_palindromes(hashes, buf, left + 1, k);
                }
            }
        }

        let mut buf = vec![0u8; n as usize];
        let mut hashes = HashSet::<PalindromeHash>::new();

        search_palindromes(&mut hashes, &mut buf, 0, k);

        // since n <= 10, so we need the value of 10! at most.
        const FACTORIALS: [i64; 11] = [
            1, // 0!
            1, // 1!
            2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, // 10!
        ];

        let mut count = 0;

        for hash in hashes.iter().copied() {
            let mut contribution = 0;

            // pick a non-zero digit as the first digit
            for idx in 1..=9usize {
                if hash[idx] == 0 {
                    continue;
                }

                // make a copy and remove the first digit from the count
                let mut hash = hash.clone();
                hash[idx] -= 1;

                // compute possible unique permutations
                // deduplicate by dividing by the factorial of each digit's count
                let mut len = 0;
                let mut div = 1;

                for (digit, digit_count) in hash.iter().copied().enumerate() {
                    if digit_count == 0 {
                        continue;
                    }

                    len += digit_count;
                    div *= FACTORIALS[digit_count as usize];
                }

                contribution += FACTORIALS[len as usize] / div;
            }

            // println!("key {:?} {}", hash, contribution);
            count += contribution;
        }

        count
    }
}

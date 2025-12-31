// Problem 1307
impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let mut found = [false; 26];
        let mut nonzero = [false; 26];
        let mut weights = [0i32; 26];

        for word in words.iter() {
            let mut factor = 1;

            for (i, ch) in word.bytes().enumerate().rev() {
                let idx = (ch - b'A') as usize;
                found[idx] = true;
                nonzero[idx] |= i == 0 && word.len() >= 2;
                weights[idx] += factor;
                factor *= 10;
            }
        }

        {
            let mut factor = 1;

            for (i, ch) in result.bytes().enumerate().rev() {
                let idx = (ch - b'A') as usize;
                found[idx] = true;
                nonzero[idx] |= i == 0 && result.len() >= 2;
                weights[idx] -= factor;
                factor *= 10;
            }
        }

        let mut data = Vec::with_capacity(10);
        for i in 0..found.len() {
            if found[i] && (weights[i] != 0 || nonzero[i]) {
                data.push((weights[i], nonzero[i]));
            }
        }

        fn dfs(data: &[(i32, bool)], used: u16, sum: i32) -> bool {
            if data.is_empty() {
                assert!(data.is_empty());
                return sum == 0;
            }

            let (weight, nonzero2) = data.first().copied().unwrap();

            let start = if nonzero2 { 1 } else { 0 };

            for digit in start..10 {
                let mask = 1 << digit;
                if used & mask == mask {
                    continue;
                }

                let new_sum = sum + weight * digit as i32;
                let solvable = dfs(&data[1..], used | mask, new_sum);
                if solvable {
                    return true;
                }
            }

            false
        }

        dfs(&data, 0, 0)
    }

    pub fn is_solvable2(words: Vec<String>, result: String) -> bool {
        let mut found = [false; 26];
        let mut nonzero = [false; 26];
        let mut weights = [0i32; 26];

        for word in words.iter() {
            let mut factor = 1;

            for (i, ch) in word.bytes().enumerate().rev() {
                let idx = (ch - b'A') as usize;
                found[idx] = true;
                nonzero[idx] |= i == 0 && word.len() >= 2;
                weights[idx] += factor;
                factor *= 10;
            }
        }

        {
            let mut factor = 1;

            for (i, ch) in result.bytes().enumerate().rev() {
                let idx = (ch - b'A') as usize;
                found[idx] = true;
                nonzero[idx] |= i == 0 && result.len() >= 2;
                weights[idx] -= factor;
                factor *= 10;
            }
        }

        let mut indexes = Vec::with_capacity(10);
        indexes.extend(found.iter().enumerate().filter_map(|(i, &seen)| {
            if seen && (nonzero[i] || weights[i] != 0) {
                // ignore seen index with zero weight if it may be zero
                // because it do not impose any restruction when assigning numbers to that index
                Some(i as u8)
            } else {
                None
            }
        }));

        indexes.sort_unstable_by_key(|&i| -weights[i as usize].abs());

        fn dfs(
            indexes: &[u8],
            weights: &[i32; 26],
            nonzero: &[bool; 26],
            used: u16,
            step: usize,
            sum: i32,
        ) -> bool {
            if step >= indexes.len() {
                return sum == 0;
            }

            let idx = indexes[step];

            let start = if nonzero[idx as usize] { 1 } else { 0 };

            for digit in start..10 {
                let mask = 1 << digit;
                if used & mask == mask {
                    continue;
                }

                let new_sum = sum + weights[idx as usize] * digit as i32;
                let solvable = dfs(indexes, weights, nonzero, used | mask, step + 1, new_sum);
                if solvable {
                    return true;
                }
            }

            false
        }

        dfs(&indexes, &weights, &nonzero, 0, 0, 0)
    }
}

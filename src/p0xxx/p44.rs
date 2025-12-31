// Problem 44
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // println!("{}", s);
        // println!("{}", p);
        let mut memo = Option::None;

        let mut i = 0;
        let mut j = 0;

        while i < s.len() {
            let actual = s.bytes().nth(i); // never be None
            let expected = p.bytes().nth(j);

            if actual == expected || expected == Some(b'?') {
                // Advance both pointers
                // println!("Advance {} {}", i, j);
                i += 1;
                j += 1;
                continue;
            }

            // If pattern is wildcard, save pattern pointer
            if expected == Some(b'*') {
                // println!("Store {} {}", i, j);
                memo = Some((i, j));
                j += 1;
                continue;
            }

            // now expected is either None or regular character, which did not match

            if let Some((i_star, j_star)) = memo {
                // println!("Restore {} {} to {} {}", i, j, i_star, j_star);

                // Use the star pattern to eat one more character
                i = i_star + 1;
                j = j_star;

                memo = Some((i, j));

                j += 1;
            } else {
                // Impossible to match
                break;
            }
        }

        while p.bytes().nth(j) == Some(b'*') {
            j += 1;
        }

        i == s.len() && j == p.len()
    }

    pub fn is_match2(s: String, p: String) -> bool {
        if s.len() == 0 {
            return p.bytes().all(|b| b == b'*');
        } else if p.len() == 0 {
            return false;
        }

        let mut memo = (0..s.len())
            .into_iter()
            .map(|_| vec![false; p.len()])
            .collect::<Vec<_>>();

        for i in 0..s.len() {
            for j in 0..p.len() {
                let expected = p.bytes().nth(j).unwrap();
                let actual = s.bytes().nth(i).unwrap();
                let matched = expected == actual || expected == b'?' || expected == b'*';

                if !matched {
                    continue;
                }

                let mut matched = i == 0 && j == 0 || expected == b'*';

                if i > 0 && j > 0 {
                    // advance both string and pattern
                    matched |= memo[i - 1][j - 1];
                }

                if j > 0 {
                    // advance pattern without eating string if previous is wildcard
                    matched |= p.bytes().nth(j - 1).unwrap() == b'*' && memo[i][j - 1];
                }

                memo[i][j] = matched;
            }

            let line = memo[i]
                .iter()
                .map(|s| if *s { "1" } else { "0" })
                .collect::<String>();
            println!("{:?}", line);
        }

        *memo.last().unwrap().last().unwrap()
    }
}

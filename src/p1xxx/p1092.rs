// Problem 1092
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let mut lcs = vec![vec![0; str2.len() + 1]; str1.len() + 1];

        for i in 0..str1.len() {
            for j in 0..str2.len() {
                if str1.bytes().nth(i).unwrap() == str2.bytes().nth(j).unwrap() {
                    lcs[i + 1][j + 1] = lcs[i][j] + 1;
                } else {
                    lcs[i + 1][j + 1] = lcs[i + 1][j].max(lcs[i][j + 1]);
                }
            }
        }

        let len = str1.len() + str2.len() - lcs[str1.len()][str2.len()];
        let mut res = Vec::<u8>::with_capacity(len);

        let mut i = str1.len();
        let mut j = str2.len();
        let mut k = res.len() - 1;

        while i > 0 && j > 0 {
            let ch1 = str1.bytes().nth(i - 1).unwrap();
            let ch2 = str2.bytes().nth(j - 1).unwrap();

            if ch1 == ch2 {
                i -= 1;
                j -= 1;
                res.push(ch1);
            } else if lcs[i][j - 1] >= lcs[i - 1][j] {
                j -= 1;
                res.push(ch2);
            } else {
                i -= 1;
                res.push(ch1);
            }
        }

        while i > 0 {
            res.push(str1.bytes().nth(i - 1).unwrap());
            i -= 1;
        }

        while j > 0 {
            res.push(str2.bytes().nth(j - 1).unwrap());
            j -= 1;
        }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}

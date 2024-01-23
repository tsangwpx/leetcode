fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 1143
impl Solution {
    pub fn longest_common_subsequence(mut text1: String, mut text2: String) -> i32 {
        if text1.len() < text2.len() {
            std::mem::swap(&mut text1, &mut text2);
        }

        let text1 = text1;
        let text2 = text2;
        assert!(text1.len() >= 1 && text1.len() <= 1000);
        assert!(text2.len() >= 1 && text2.len() <= 1000);


        let mut dp0 = [0u16; 1024];
        let mut dp1 = dp0.clone();

        for i in 0..text1.len() {
            for j in 0..text2.len() {
                dp1[j + 1] = if text1.bytes().nth(i) == text2.bytes().nth(j) {
                    dp0[j] + 1
                } else {
                    dp0[j + 1].max(dp1[j])
                };
            }

            // println!("{:?}", dp1);

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0[text2.len()] as i32
    }

    pub fn longest_common_subsequence2(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0u16; text2.len() + 1]; text1.len() + 1];

        for i in 0..text1.len() {
            for j in 0..text2.len() {
                let count;

                if text1.bytes().nth(i) == text2.bytes().nth(j) {
                    count = dp[i][j] + 1;
                } else {
                    count = dp[i + 1][j].max(dp[i][j + 1]);
                }
                dp[i + 1][j + 1] = count;
            }
        }

        for row in dp.iter() {
            println!("{:?}", row);
        }

        dp[text1.len()][text2.len()] as i32
    }
}

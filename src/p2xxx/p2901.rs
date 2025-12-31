// Problem 2901
impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        assert!(groups.len() == n);

        let mut parents = (0..n).collect::<Vec<usize>>();
        let mut dp = vec![1; n];
        let mut max_idx = 0;

        for i in 0..n {
            for j in 0..i {
                if groups[i] == groups[j] || dp[i] >= dp[j] + 1 {
                    continue;
                }
                let a = &words[i];
                let b = &words[j];
                if a.len() != b.len() {
                    continue;
                }

                let mut count = 0;
                for (ch1, ch2) in a.bytes().zip(b.bytes()) {
                    if ch1 != ch2 {
                        count += 1;
                    }

                    if count >= 2 {
                        break;
                    }
                }

                if count == 1 {
                    parents[i] = j;
                    dp[i] = dp[j] + 1;

                    if dp[i] > dp[max_idx] {
                        max_idx = i;
                    }
                }
            }
        }

        let mut trace = vec![max_idx];

        loop {
            let idx = trace.last().copied().unwrap();
            let parent = parents[idx];
            if parent == idx {
                break;
            }
            trace.push(parent);
        }

        trace.reverse();

        trace
            .into_iter()
            .map(|s| words[s].clone())
            .collect::<Vec<String>>()
    }
}

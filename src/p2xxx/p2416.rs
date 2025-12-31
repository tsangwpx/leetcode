// Problem 2416
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        #[derive(Default, Debug, Clone, Copy)]
        struct Node {
            count: i32,
            children: [usize; 26],
        }

        let mut trie = vec![];
        trie.push(Node::default());

        for word in words.iter() {
            let mut node = 0;

            for ch in word.bytes() {
                let ch = (ch - b'a') as usize;
                let mut next = trie[node].children[ch];

                if next == 0 {
                    next = trie.len();
                    trie[node].children[ch] = next;
                    trie.push(Node::default());
                }

                trie[next].count += 1;

                node = next;
            }
        }

        words
            .iter()
            .map(|word| {
                let mut count = 0;
                let mut node = 0;
                for ch in word.bytes() {
                    let ch = (ch - b'a') as usize;

                    node = trie[node].children[ch];
                    count += trie[node].count;
                }
                count
            })
            .collect::<Vec<_>>()
    }
}

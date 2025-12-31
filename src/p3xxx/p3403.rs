// Problem 3403
impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        // slow...
        if num_friends == 1 {
            return word;
        }

        let len = word.len() - num_friends as usize + 1;
        let mut idx = 0;

        for idx2 in 1..word.len() {
            if word
                .bytes()
                .skip(idx2)
                .take(len)
                .cmp(word.bytes().skip(idx).take(len))
                .is_gt()
            {
                idx = idx2;
            }
        }

        word.chars().skip(idx).take(len).collect::<String>()
    }
}

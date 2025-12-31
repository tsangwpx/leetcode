// Problem 3324
impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut res = vec![];
        let mut tmp = String::with_capacity(target.len());

        for ch in target.chars() {
            for ch2 in 'a'..=ch {
                let mut item = String::with_capacity(tmp.len() + 1);
                item.push_str(&tmp);
                item.push(ch2);
                res.push(item);
            }

            tmp.push(ch);
        }

        res
    }
}

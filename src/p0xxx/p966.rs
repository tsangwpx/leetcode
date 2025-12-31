// Problem 966
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut exact_map = HashMap::new();
        let mut ci_map = HashMap::new();
        let mut vowel_map = HashMap::new();

        #[inline]
        fn to_ci(s: &String) -> String {
            s.to_lowercase()
        }

        #[inline]
        fn to_any_vowel(s: &String) -> String {
            let r = s
                .to_lowercase()
                .chars()
                .map(|ch| {
                    if matches!(
                        ch,
                        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
                    ) {
                        '?'
                    } else {
                        ch
                    }
                })
                .collect::<String>();
            // println!("{:?} {:?}", s, r);

            r
        }

        for word in wordlist.iter() {
            exact_map.insert(word, word);

            let lower = to_ci(word);
            ci_map.entry(lower).or_insert(word);

            let any_vowel = to_any_vowel(word);
            vowel_map.entry(any_vowel).or_insert(word);
        }

        let mut res: Vec<String> = vec![];

        for query in queries.iter() {
            let item = if let Some(found) = exact_map.get(query) {
                String::clone(&found)
            } else if let Some(found) = ci_map.get(&to_ci(query)) {
                String::clone(&found)
            } else if let Some(found) = vowel_map.get(&to_any_vowel(query)) {
                String::clone(&found)
            } else {
                "".to_owned()
            };

            res.push(item);
        }

        res
    }
}

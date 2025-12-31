// Problem 1733
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut people = HashSet::new();
        let languages = languages
            .into_iter()
            .map(|row| {
                let mut langs = HashSet::<i32>::new();
                langs.extend(row.iter());
                langs
            })
            .collect::<Vec<_>>();

        for row in friendships.iter() {
            let &[u, v] = row.as_slice() else {
                panic!();
            };
            let u = u - 1;
            let v = v - 1;

            let empty = languages[u as usize]
                .intersection(&languages[v as usize])
                .count()
                == 0;

            if empty {
                people.insert(u);
                people.insert(v);
            }
        }

        let mut maximum = n as i32;

        for i in 0..n {
            let mut count = 0;
            for u in people.iter().copied() {
                if !languages[u as usize].contains(&(i as i32)) {
                    count += 1;
                }
            }
            maximum = maximum.min(count);
        }

        maximum
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut indices = (0..people.len()).collect::<Vec<usize>>();
        indices.sort_unstable_by_key(|&a| {
            assert_eq!(people[a].len(), 2);
            (-people[a][0], people[a][1])
        });

        let mut queue = Vec::with_capacity(people.len());
        for i in indices {
            let person = &people[i];
            assert_eq!(person.len(), 2);
            let h = person[0];
            let k = person[1];

            queue.insert(k as usize, i);
        }
        let mut people = people;
        let mut res = vec![vec![]; people.len()];

        for (i, &j) in queue.iter().enumerate() {
            std::mem::swap(&mut res[i], &mut people[j]);
        }

        res
    }
}

struct Solution {}

fn main() {
    println!("Hello World");
}
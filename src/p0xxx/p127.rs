// Problem 127
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        use std::rc::Rc;

        let mut word_list = word_list
            .into_iter()
            .map(|s| Rc::new(s))
            .collect::<Vec<_>>();

        let mut word_table = word_list
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect::<HashMap<_, _>>();

        let begin_index = {
            let begin_word = Rc::new(begin_word);
            *word_table.entry(begin_word.clone()).or_insert_with(|| {
                let word_index = word_list.len();
                word_list.push(begin_word.clone());
                word_index
            })
        };

        let end_index;
        if let Some(index) = word_table.get(&end_word) {
            end_index = *index;
        } else {
            return 0;
        }
        drop(word_table);

        let mut edges = vec![vec![0usize; 0]; word_list.len()];

        for (word_index, word) in word_list.iter().enumerate() {
            for (word2_index, word2) in word_list.iter().enumerate().skip(word_index + 1) {
                let change_count = word
                    .bytes()
                    .zip(word2.bytes())
                    .filter(|(a, b)| a != b)
                    .count();
                if change_count == 1 {
                    edges[word_index].push(word2_index);
                    edges[word2_index].push(word_index);
                }
            }
        }
        // println!("{:?}", word_list);
        drop(word_list);

        type Distance = i8;

        let mut distances = vec![Distance::MAX; edges.len()];
        distances[begin_index] = 0;

        let mut heap = BinaryHeap::<(Reverse<Distance>, usize)>::new();
        heap.reserve(edges.len());
        heap.extend(distances.iter().enumerate().map(|(i, &d)| (Reverse(d), i)));

        while let Some((Reverse(dist), node_index)) = heap.pop() {
            let dist2 = distances[node_index];
            if dist2 == Distance::MAX || dist2 != dist {
                continue;
            }
            // println!("{} {}", dist, node_index);

            for &friend_index in edges[node_index].iter() {
                let friend_dist = distances[friend_index];
                if dist + 1 < friend_dist {
                    distances[friend_index] = dist + 1;
                    heap.push((Reverse(distances[friend_index]), friend_index));
                }
            }
        }

        if distances[end_index] == Distance::MAX {
            0
        } else {
            distances[end_index] as i32 + 1
        }
    }
}

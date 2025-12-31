// Problem 347
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        assert!(k >= 1);
        assert!(nums.len() >= 1);

        use std::cmp::Reverse;
        use std::collections::HashMap;

        let mut counter = HashMap::new();

        nums.iter().for_each(|&number| {
            counter
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let mut pairs = counter.into_iter().collect::<Vec<_>>();

        let k_th_index = (k as usize - 1).min(pairs.len() - 1);
        pairs.select_nth_unstable_by_key(k_th_index, |&(_, count)| Reverse(count));

        pairs
            .into_iter()
            .take(k as usize)
            .map(|(number, _)| number)
            .collect()
    }

    pub fn top_k_frequent2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        assert!(k >= 1);
        assert!(nums.len() >= 1);

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let mut counter = HashMap::new();

        nums.iter().for_each(|&number| {
            counter
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let k_plus_one = k as usize + 1;

        let mut minheap = BinaryHeap::with_capacity(k_plus_one);
        let mut it = counter.into_iter();

        // Fill the heap until its size reach k + 1
        while minheap.len() < k_plus_one {
            match it.next() {
                Some((number, count)) => minheap.push((Reverse(count), number)),
                None => break,
            }
        }

        // replace the (k+1)-th frequent number
        while let Some((number, count)) = it.next() {
            let mut peek = minheap.peek_mut().unwrap();
            *peek = (Reverse(count), number);
        }

        // Pop the k+1 in heap
        if minheap.len() == k_plus_one {
            minheap.pop();
        }

        // the top k numbers
        minheap.into_iter().map(|(_, number)| number).collect()
    }
}

// Problem 3362
impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut queries = queries;
        queries.sort_unstable_by_key(|s| s[0]);

        let mut available = BinaryHeap::new();
        let mut used = BinaryHeap::new();

        let mut count = 0;
        let mut j = 0;

        for (idx, item) in nums.iter().copied().enumerate() {
            while j < queries.len() && queries[j][0] as usize == idx {
                available.push(queries[j][1] as usize);
                j += 1;
            }

            let item = item as usize;
            while item > used.len() {
                if let Some(idx2) = available.pop() {
                    if idx2 >= idx {
                        used.push(Reverse(idx2));
                        count += 1;
                    }
                } else {
                    break;
                }
            }

            // println!("{} {} {:?} {:?}", idx, item, available, used);

            if item > used.len() {
                return -1;
            }

            while let Some(Reverse(idx2)) = used.peek().copied() {
                if idx >= idx2 {
                    used.pop();
                } else {
                    break;
                }
            }
        }

        (queries.len() - count) as i32
    }
}

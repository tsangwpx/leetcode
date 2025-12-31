// Problem 2054
impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        events.sort_by_key(|s| s[0]);

        let mut heap = BinaryHeap::new();

        let mut best = 0;
        let mut res = 0;

        for row in events.iter() {
            let &[start_time, end_time, value] = row.as_slice() else {
                panic!("bad format");
            };

            while let Some((Reverse(time), value)) = heap.peek().copied() {
                if time >= start_time {
                    break;
                }

                best = best.max(value);
                heap.pop();
            }

            res = res.max(value + best);

            heap.push((Reverse(end_time), value));
        }

        res
    }
}

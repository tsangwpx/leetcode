// Problem 757
impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|pair| (pair[1], pair[0]));

        let mut chosen = 0;
        let mut stash = vec![];

        for pair in intervals.iter() {
            let &[left, right] = pair.as_slice() else {
                panic!();
            };

            stash.retain(|&item| {
                if left <= item && item <= right {
                    true
                } else {
                    chosen += 1;
                    false
                }
            });

            let mut target = right;

            while target >= left && stash.len() < 2 {
                if !stash.contains(&target) {
                    stash.push(target);
                }

                target -= 1;
            }
        }

        chosen + stash.len() as i32
    }
}

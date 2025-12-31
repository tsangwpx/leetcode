// Problem 1298
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = status.len();
        assert!(
            status.len() == n
                && candies.len() == n
                && keys.len() == n
                && contained_boxes.len() == n
        );

        // boxes that are going to visit.
        let mut pending = vec![];

        // boxes that have been visited.
        let mut visited = vec![false; n];

        // found boxes without keys nor open
        let mut unused_boxes = vec![false; n];

        // found keys without boxes
        let mut unused_keys = vec![false; n];

        for idx in initial_boxes.iter().copied() {
            let idx = idx as usize;

            if status[idx] == 1 {
                pending.push(idx);
            } else {
                unused_boxes[idx] = true;
            }
        }

        let mut collected = 0;

        while let Some(idx) = pending.pop() {
            if visited[idx] {
                continue;
            }

            visited[idx] = true;
            collected += candies[idx];

            for idx2 in keys[idx].iter().copied() {
                let idx2 = idx2 as usize;

                if unused_boxes[idx2] {
                    // the box is ready
                    unused_boxes[idx2] = false;
                    pending.push(idx2);
                } else {
                    // the box is missing
                    unused_keys[idx2] = true;
                }
            }

            for idx2 in contained_boxes[idx].iter().copied() {
                let idx2 = idx2 as usize;

                if status[idx2] == 1 || unused_keys[idx2] {
                    // the box is ready, consume the key anyway
                    unused_keys[idx2] = false;
                    pending.push(idx2);
                } else {
                    // save the box for later
                    unused_boxes[idx2] = true;
                }
            }
        }

        collected
    }
}

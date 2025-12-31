// Problem 1857
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        let n = colors.len();
        let mut successors = vec![vec![]; n];
        let mut predecessors = vec![0; n];
        let mut max_colors = vec![[0; 26]; n];

        for row in edges.iter() {
            let &[src, dst] = row.as_slice() else {
                unreachable!();
            };

            let src = src as usize;
            let dst = dst as usize;

            predecessors[dst] += 1;
            successors[src].push(dst);
        }

        let mut deque = VecDeque::new();
        let mut max_color_value = 0;
        let mut done = 0;

        for (idx, count) in predecessors.iter().copied().enumerate() {
            if count == 0 {
                deque.push_back(idx);
            }
        }

        while let Some(idx) = deque.pop_front() {
            done += 1;

            let node_color = (colors.bytes().nth(idx).unwrap() - b'a') as usize;
            max_colors[idx][node_color] += 1;
            max_color_value = max_color_value.max(max_colors[idx][node_color]);

            for next in successors[idx].iter().copied() {
                predecessors[next] -= 1;
                if predecessors[next] == 0 {
                    deque.push_back(next);
                }

                for color in 0..26 {
                    max_colors[next][color] = max_colors[next][color].max(max_colors[idx][color]);
                }
            }
        }

        if done == n { max_color_value } else { -1 }
    }
}

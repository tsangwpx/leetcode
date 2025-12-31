// Problem 3203
impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        use std::collections::VecDeque;

        fn min_height_diameter(edges: &Vec<Vec<i32>>) -> (i32, i32) {
            if edges.is_empty() {
                return (0, 0);
            }

            let n = edges.len() + 1;
            let mut connected = vec![HashSet::new(); n];
            let mut heights = vec![0; n];

            for link in edges.iter() {
                let &[u, v] = link.as_slice() else {
                    panic!("Bad link {:?}", link);
                };

                let u = u as usize;
                let v = v as usize;
                if u >= n || v >= n {
                    panic!("Bad link {:?}", link);
                }

                connected[u].insert(v);
                connected[v].insert(u);
            }

            let mut queue = connected
                .iter()
                .enumerate()
                .filter_map(|(idx, neighbors)| match neighbors.len() {
                    1 => Some(idx),
                    _ => None,
                })
                .collect::<VecDeque<_>>();

            let mut last_removed_nodes = vec![];

            while !queue.is_empty() {
                last_removed_nodes.clear();
                last_removed_nodes.extend(queue.iter().copied());

                for _ in 0..queue.len() {
                    let Some(idx) = queue.pop_front() else {
                        panic!("Queue exhausted unexpectedly");
                    };

                    if let Some(parent) = connected[idx].iter().copied().next() {
                        heights[parent] = heights[parent].max(heights[idx] + 1);

                        connected[parent].remove(&idx);

                        if connected[parent].len() == 1 {
                            queue.push_back(parent);
                        }
                    }
                }
            }

            if last_removed_nodes.is_empty() {
                (0, 0)
            } else if last_removed_nodes.len() == 1 {
                let idx = last_removed_nodes[0];
                let height = heights[idx];
                (height, height * 2)
            } else {
                let idx = last_removed_nodes[0];
                let height = heights[idx];
                (height + 1, height * 2 + 1)
            }
        }

        let (h1, d1) = min_height_diameter(&edges1);
        let (h2, d2) = min_height_diameter(&edges2);

        d1.max(d2).max(h1 + h2 + 1)
    }
}

// Problem 3372
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        fn build_adjacent(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let len = edges.len() + 1;
            let mut adjacent = vec![vec![]; len];

            for edge in edges.iter() {
                let &[u, v] = edge.as_slice() else {
                    unreachable!();
                };

                adjacent[u as usize].push(v);
                adjacent[v as usize].push(u);
            }

            adjacent
        }

        fn count_nodes(adjacent: &Vec<Vec<i32>>, node: usize, parent: usize, k: i32) -> i32 {
            match k.cmp(&0) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Greater => {
                    let mut step = 1;
                    if k > 0 {
                        for friend in adjacent[node].iter().copied() {
                            let friend = friend as usize;
                            if friend == parent {
                                continue;
                            }

                            step += count_nodes(adjacent, friend, node, k - 1);
                        }
                    }

                    step
                }
            }
        }

        let adj1 = build_adjacent(&edges1);
        let adj2 = build_adjacent(&edges2);

        let mut foreign_nodes = 0;
        for idx in 0..adj2.len() {
            foreign_nodes = foreign_nodes.max(count_nodes(&adj2, idx, idx, k - 1));
        }

        let mut res = vec![foreign_nodes; adj1.len()];
        for idx in 0..adj1.len() {
            res[idx] += count_nodes(&adj1, idx, idx, k);
        }

        res
    }
}

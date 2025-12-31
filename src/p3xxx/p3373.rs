// Problem 3373
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        /*
         * 1. Build the adjacent list of the graph
         * 2. Traverse the graph from arbitrary node, and assign each node a label, even or odd.
         * 3. Count the number of even and odd nodes.
         * 4. The distances between nodes with the same label must be even.
         *      This give the number of even distance nodes in the first tree.
         * 5. We have the freedom to connect any node between first and second tree,
         *      so we can always pick the label with more nodes in the second tree,
         *      and add this to the count in the first tree
         */
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

        #[derive(Clone, Copy, PartialEq)]
        enum Parity {
            Even,
            Odd,
        }

        fn dfs(
            adjacent: &Vec<Vec<i32>>,
            node: usize,
            parent: usize,
            parity: Parity,
            parities: &mut Vec<Parity>,
        ) -> (i32, i32) {
            let (mut even, mut odd) = (0, 0);

            if parity == Parity::Even {
                even += 1;
            } else {
                odd += 1;
            }

            parities[node] = parity;

            for friend in adjacent[node].iter().copied() {
                let friend = friend as usize;
                if friend == parent {
                    continue;
                }

                let friend_parity = match parity {
                    Parity::Even => Parity::Odd,
                    Parity::Odd => Parity::Even,
                };

                let (even2, odd2) = dfs(adjacent, friend, node, friend_parity, parities);
                even += even2;
                odd += odd2;
            }

            (even, odd)
        }

        let adj1 = build_adjacent(&edges1);
        let adj2 = build_adjacent(&edges2);

        let mut partities1 = vec![Parity::Even; adj1.len()];
        let mut partities2 = vec![Parity::Even; adj2.len()];

        let (even2, odd2) = dfs(&adj2, 0, 0, Parity::Even, &mut partities2);
        // println!("{} {}", even2, odd2);
        let max2 = even2.max(odd2);
        let (even1, odd1) = dfs(&adj1, 0, 0, Parity::Even, &mut partities1);
        // println!("{} {}", even1, odd1);

        let mut res = vec![0; adj1.len()];
        for idx in 0..res.len() {
            res[idx] = match partities1[idx] {
                Parity::Even => even1 + max2,
                Parity::Odd => odd1 + max2,
            }
        }

        res
    }
}

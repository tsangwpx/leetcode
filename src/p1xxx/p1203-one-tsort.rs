// Problem 1203
impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let m = m as usize;
        let total_nodes = n + 2 * m; // need two more nodes for each group
        assert!(n == group.len() && n == before_items.len());
        assert!(m >= 1);

        let mut successors = vec![vec![]; total_nodes];
        let mut predecessors = vec![0u32; total_nodes];

        for (item, group) in group.iter().copied().enumerate() {
            if group < 0 {
                continue;
            }
            let group = group as usize;
            let group_start = n + 2 * group;
            let group_stop = group_start + 1;

            successors[item].push(group_stop as u32);
            successors[group_start].push(item as u32);

            predecessors[item] += 1;
            predecessors[group_stop] += 1;
        }

        for (item, item_predecessors) in before_items.iter().enumerate() {
            let item_group = group[item];

            for prior in item_predecessors.iter().copied() {
                let prior = prior as usize;
                let prior_group = group[prior];

                if item_group == prior_group {
                    successors[prior].push(item as u32);
                    predecessors[item] += 1;
                } else {
                    let item_delegate = if item_group >= 0 {
                        n + 2 * item_group as usize
                    } else {
                        item
                    };

                    let prior_delegate = if prior_group >= 0 {
                        n + 2 * prior_group as usize + 1
                    } else {
                        prior
                    };

                    successors[prior_delegate].push(item_delegate as u32);
                    predecessors[item_delegate] += 1;
                }
            }
        }

        let mut res = Vec::<i32>::with_capacity(n);

        let mut ready_nodes = Vec::with_capacity(n);
        ready_nodes.extend(
            predecessors
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(node, count)| if count == 0 { Some(node) } else { None }),
        );

        while let Some(node) = ready_nodes.pop() {
            // println!(
            //     "Node: {}; group {:?}",
            //     node,
            //     if node >= n {
            //         Some(((node - n) / 2, (node - n) % 2))
            //     } else {
            //         None
            //     }
            // );
            for successor in successors[node].iter().copied() {
                let successor = successor as usize;
                predecessors[successor] -= 1;
                if predecessors[successor] == 0 {
                    ready_nodes.push(successor);
                }
            }

            if node < n {
                res.push(node as i32);
            }
        }

        if res.len() == n { res } else { vec![] }
    }
}

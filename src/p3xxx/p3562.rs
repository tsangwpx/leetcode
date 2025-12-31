// Problem 3562
impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let n = n as usize;
        let mut children = vec![vec![]; n];

        for relationship in hierarchy.iter() {
            let boss = relationship[0] - 1;
            let employee = relationship[1] - 1;
            children[boss as usize].push(employee);
        }

        let budget = budget as usize;

        fn dfs(
            n: usize,
            present: &Vec<i32>,
            future: &Vec<i32>,
            budget: usize,
            children: &Vec<Vec<i32>>,
            node: usize,
        ) -> (Vec<i32>, Vec<i32>) {
            // dp0 is the maximum child profits given this node is not bought
            // dq0 is the maximum child profits given this node is bought

            let mut dp0 = vec![0; budget + 1];
            let mut dq0 = vec![0; budget + 1];

            for child in children[node].iter().copied() {
                let mut dp1 = vec![0; budget + 1];
                let mut dq1 = vec![0; budget + 1];

                let (cp, cq) = dfs(n, present, future, budget, children, child as usize);

                for c1 in 0..dp1.len() {
                    // c1 is the cost spent in previously children

                    for c2 in 0..cp.len() {
                        // c2 is the cost spent in this child
                        let cost = c1 + c2;

                        if c1 + c2 > budget {
                            break;
                        }

                        dp1[cost] = dp1[cost].max(dp0[c1] + cp[c2]);
                    }
                }

                for c1 in 0..dq1.len() {
                    // c1 is the cost spent in previously children + this node

                    for c2 in 0..cq.len() {
                        // c2 is the cost spent in this child
                        let cost = c1 + c2;

                        if c1 + c2 > budget {
                            break;
                        }

                        dq1[cost] = dq1[cost].max(dq0[c1] + cq[c2]);
                    }
                }

                std::mem::swap(&mut dp0, &mut dp1);
                std::mem::swap(&mut dq0, &mut dq1);
            }

            // the result of this node depends on the parent node is bought or not.
            // profits0 = maximum profits of this node if parent is not bought
            // profits1 = maximum profits of this node if parent is bought

            // The lower bound for both profits0 and profits1 is dp0
            let mut profits0 = dp0;
            let mut profits1 = profits0.clone();

            // parent is not bought and me is bought
            let cost = present[node] as usize;
            let node_profit = future[node] - present[node];

            for c1 in cost..profits0.len() {
                profits0[c1] = profits0[c1].max(dq0[c1 - cost] + node_profit);
            }

            // parent is bought and me is bought
            let cost = present[node] as usize / 2;
            let node_profit = future[node] - present[node] / 2;
            for c1 in cost..profits1.len() {
                profits1[c1] = profits1[c1].max(dq0[c1 - cost] + node_profit);
            }

            // println!("node {} {:?}", node, profits0);
            // println!("node {} {:?}", node, profits1);

            (profits0, profits1)
        }

        let (profits, _) = dfs(n, &present, &future, budget, &children, 0);

        profits[budget]
    }
}

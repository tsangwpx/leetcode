use std::cell::Cell;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        use ::std::collections::HashMap;
        use ::std::hash::Hash;
        use ::std::rc::Rc;
        use std::cmp::{max, min};

        #[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
        struct TreeKey {
            node: u16,
            parent: u16,
        }

        impl TreeKey {
            fn new(node: u16, parent: u16) -> Self {
                Self { node, parent }
            }
        }

        #[derive(Debug)]
        struct TreeInfo {
            xor: i32,
            descendants: Vec<i32>,
        }

        type XORTable = HashMap<TreeKey, Rc<TreeInfo>>;
        let mut xor_table: XORTable = HashMap::with_capacity(edges.len() * 2);
        let mut neighbours: Vec<Vec<u16>> = vec![vec![]; nums.len()];
        let mut ans = i32::MAX;

        // Create the neighbours list for each node
        edges.iter().for_each(|edge| {
            assert_eq!(edge.len(), 2);
            neighbours[edge[0] as usize].push(edge[1] as u16);
            neighbours[edge[1] as usize].push(edge[0] as u16);
        });

        /// Find TreeInfo
        fn find_tree_info(
            xor_table: &mut XORTable,
            neighbours: &Vec<Vec<u16>>,
            nums: &Vec<i32>,
            key: TreeKey,
        ) -> Rc<TreeInfo> {
            if let Some(tree_info) = xor_table.get(&key) {
                return Rc::clone(tree_info);
            }

            let node = key.node as usize;
            let parent = key.parent;
            let mut xor = nums[node];
            let mut descendants = Vec::new();

            for &child in neighbours[node].iter() {
                if child != parent {
                    let child_info =
                        find_tree_info(xor_table, neighbours, nums, TreeKey::new(child, key.node));
                    xor ^= child_info.xor;
                    descendants.reserve(1 + child_info.descendants.len());
                    descendants.push(child_info.xor);
                    descendants.extend_from_slice(child_info.descendants.as_slice());
                }
            }
            let rc = Rc::new(TreeInfo { xor, descendants });
            xor_table.insert(key, rc.clone());
            rc
        }

        #[inline(always)]
        fn xor_diff(a: i32, b: i32, c: i32) -> i32 {
            a.max(b.max(c)) - a.min(b.min(c))
        }

        edges.iter().for_each(|edge| {
            assert_eq!(edge.len(), 2);
            let a = edge[0] as u16;
            let b = edge[1] as u16;

            // Break the edge a <-> b and result in two trees
            let tree_a = find_tree_info(&mut xor_table, &neighbours, &nums, TreeKey::new(a, b));
            let tree_b = find_tree_info(&mut xor_table, &neighbours, &nums, TreeKey::new(b, a));

            // Break one edge in tree_b to have tree_c
            for &xor_c in tree_b.descendants.iter() {
                let xor_a = tree_a.xor;
                let xor_b = tree_b.xor ^ xor_c; // remove xor_c from xor_b
                ans = min(ans, xor_diff(xor_a, xor_b, xor_c));
            }

            // Break one edge in tree_a to have tree_c
            for &xor_c in tree_a.descendants.iter() {
                let xor_b = tree_b.xor;
                let xor_a = tree_a.xor ^ xor_c; // remove xor_c from xor_a
                ans = min(ans, xor_diff(xor_a, xor_b, xor_c));
            }
        });

        ans
    }
}

fn main() {
    Solution::minimum_score(
        vec![1, 5, 5, 4, 11],
        vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
    );
    println!("Hello World");
}

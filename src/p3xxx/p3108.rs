mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 3108
impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        #[derive(Debug, Clone, Copy)]
        struct Node {
            parent: i32,
            cost: i32,
        }

        let mut nodes = (0..n)
            .map(|s| Node {
                parent: s,
                cost: i32::MAX,
            })
            .collect::<Vec<_>>();

        fn find_root(nodes: &[Node], src: i32) -> i32 {
            // Find root of a node

            let parent = nodes[src as usize].parent;

            if src == parent {
                src
            } else {
                find_root(nodes, parent)
            }
        }

        fn union(nodes: &mut [Node], a: i32, b: i32) -> i32 {
            // Union two groups and returns the new root

            let a_root = find_root(nodes, a);
            let b_root = find_root(nodes, b);

            let (root, other) = (a_root.min(b_root), a_root.max(b_root));

            nodes[a as usize].parent = root;
            nodes[b as usize].parent = root;
            nodes[other as usize].parent = root;
            nodes[root as usize].cost &= nodes[other as usize].cost;
            root
        }

        for item in edges {
            let &[u, v, w] = item.as_slice() else {
                unreachable!("Bad edge")
            };
            let root = union(&mut nodes, u, v);
            nodes[root as usize].cost &= w;
        }

        // println!("{:?}", nodes);

        let mut res = vec![-1; query.len()];

        for (idx, item) in query.into_iter().enumerate() {
            let &[u, v] = item.as_slice() else {
                unreachable!("Bad query")
            };

            if u == v {
                res[idx] = 0;
                continue;
            }

            let u = find_root(&nodes, u);
            let v = find_root(&nodes, v);

            if u == v {
                res[idx] = nodes[u as usize].cost;
            }
        }

        res
    }
}

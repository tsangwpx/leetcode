mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

extern crate rand;

// Problem 1579
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        const ALICE: i32 = 1;
        const BOB: i32 = 2;
        const BOTH: i32 = 3;

        let mut parents = (0..n).collect::<Vec<i32>>();
        // start from any node
        let mut alice_reachable = 1;
        let mut bob_reachable = 1;
        let mut removed = 0;

        #[inline]
        fn find(parents: &mut Vec<i32>, node: i32) -> i32 {
            let mut root = parents[node as usize];

            if parents[root as usize] != root {
                // If the root is no longer a root, update the root.
                root = find(parents, root);
                parents[node as usize] = root;
            }

            root
        }

        #[inline]
        fn merge(parents: &mut Vec<i32>, mut u: i32, mut v: i32) -> bool {
            u = find(parents, u);
            v = find(parents, v);

            if u != v {
                // assert!(u < v);
                parents[v as usize] = u;
                true
            } else {
                false
            }
        }

        macro_rules! run {
            ($who:expr, $parents: ident, $yes:block, $no: block) => {
                for row in edges.iter() {
                    let &[type_, mut u, mut v] = row.as_slice() else {
                        unreachable!();
                    };

                    if type_ == $who {
                        u -= 1;
                        v -= 1;

                        if merge(&mut $parents, u, v) {
                            $yes
                        } else {
                            $no
                        }
                    }
                }
            };
        }

        run!(
            BOTH,
            parents,
            {
                alice_reachable += 1;
                bob_reachable += 1;
            },
            {
                removed += 1;
            }
        );

        let mut alice_parents = parents;
        let mut bob_parents = alice_parents.clone();

        run!(
            ALICE,
            alice_parents,
            {
                alice_reachable += 1;
            },
            {
                removed += 1;
            }
        );

        run!(
            BOB,
            bob_parents,
            {
                bob_reachable += 1;
            },
            {
                removed += 1;
            }
        );

        if alice_reachable == n && bob_reachable == n {
            removed
        } else {
            -1
        }
    }
}

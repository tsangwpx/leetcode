// Problem 1970
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        #[derive(Clone, Copy, Debug)]
        struct GroupLimits {
            // record the extreme value in a group
            leftmost: i32,
            rightmost: i32,
        }

        #[derive(Default, Debug)]
        struct UnionFind {
            /**
             * Oh, cells.len() == row * col
             * so preallocate the parents as Vec<usize> may be better instead.
             * Cell position (r, c) can be numbered by `r * col + c`
             */
            parents: HashMap<(i32, i32), (i32, i32)>,
            limits: HashMap<(i32, i32), GroupLimits>,
        }

        impl UnionFind {
            fn ensure_group(&mut self, pos: (i32, i32)) {
                let found = self.parents.contains_key(&pos);

                if !found {
                    self.parents.insert(pos, pos);
                    let (_, col) = pos;
                    self.limits.insert(
                        pos,
                        GroupLimits {
                            leftmost: col,
                            rightmost: col,
                        },
                    );
                }
            }

            fn find_group(&mut self, pos: (i32, i32)) -> Option<(i32, i32)> {
                if let Some(parent) = self.parents.get(&pos).copied() {
                    if parent == pos {
                        return Some(pos);
                    }

                    if let Some(parent2) = self.find_group(parent) {
                        if parent != parent2 {
                            self.parents.insert(pos, parent2);
                        }

                        return Some(parent2);
                    }
                }

                None
            }

            fn union(&mut self, pos1: (i32, i32), pos2: (i32, i32)) -> Option<(i32, i32)> {
                let Some(root1) = self.find_group(pos1) else {
                    return None;
                };

                let Some(root2) = self.find_group(pos2) else {
                    return None;
                };

                if root1 == root2 {
                    return Some(root1);
                }

                let new_root = root1.min(root2);
                let old_root = root1.max(root2);

                self.parents
                    .insert(old_root, new_root)
                    .expect("old_root does not exist");

                let old_limits = self.limits.remove(&old_root).expect("logic error");
                let new_limits = self.limits.get_mut(&new_root).expect("logic error");

                new_limits.leftmost = new_limits.leftmost.min(old_limits.leftmost);
                new_limits.rightmost = new_limits.rightmost.max(old_limits.rightmost);

                Some(new_root)
            }
        }

        let mut water = UnionFind::default();

        for (day, cell) in cells.iter().enumerate() {
            let &[r, c] = cell.as_slice() else {
                panic!("bad format");
            };

            let center = (r, c);
            water.ensure_group(center);

            for neighbor in [
                (r - 1, c - 1),
                (r - 1, c),
                (r - 1, c + 1),
                (r, c - 1),
                (r, c + 1),
                (r + 1, c - 1),
                (r + 1, c),
                (r + 1, c + 1),
            ] {
                water.union(neighbor, center);
            }

            let root = water.find_group(center).expect("logic error");
            let limits = water.limits.get(&root).copied().unwrap();

            let width = limits.rightmost + 1 - limits.leftmost;
            // println!("Day {} {:?}", day + 1, limits);
            if width >= col {
                return day as i32;
            }
        }

        unreachable!("bad input")
    }
}

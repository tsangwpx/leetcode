use core::slice::SlicePattern;

// Problem 1203
impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let n = n as usize;
        assert!(n == group.len() && n == before_items.len());
        assert!(m >= 1);

        // Assign group to items without group.
        let mut group_items = vec![vec![]; n as usize];

        let mut m = m as usize;
        let mut group = group;

        for (item, group) in group.iter_mut().enumerate() {
            if *group < 0 {
                if m >= group_items.len() {
                    group_items.push(vec![item as i32]);
                } else {
                    group_items[m].push(item as i32);
                }

                *group = m as i32;
                m += 1;
            } else {
                group_items[*group as usize].push(item as i32);
            }
        }

        let m = m as usize;
        let group = group;

        let mut item_successors = vec![HashSet::<i32>::new(); n];
        let mut item_predecessors = vec![0; n];
        let mut group_successors = vec![HashSet::<i32>::new(); m];
        let mut group_predecessors = vec![0; m];

        for (successor, predecessors) in before_items.iter().enumerate() {
            let successor_group = group[successor];

            for predecessor in predecessors.iter().copied() {
                let predecessor_group = group[predecessor as usize];

                if successor_group == predecessor_group {
                    if item_successors[predecessor as usize].insert(successor as i32) {
                        item_predecessors[successor] += 1;
                    }
                } else {
                    if group_successors[predecessor_group as usize].insert(successor_group) {
                        group_predecessors[successor_group as usize] += 1;
                    }
                }
            }
        }

        let mut remaining_items = n;
        let mut remaining_groups = m;
        let mut res = Vec::<i32>::with_capacity(n);

        let mut ready_group = group_predecessors
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(group, count)| if count == 0 { Some(group) } else { None })
            .collect::<Vec<_>>();
        let mut ready_items = Vec::<i32>::with_capacity(n);

        // println!("Ready: {:?}", ready_group);

        'outer: while let Some(group) = ready_group.pop() {
            // println!("Group {}: {:?}", group, group_successors[group]);

            remaining_groups -= 1;

            for successor in group_successors[group].iter().copied() {
                group_predecessors[successor as usize] -= 1;
                if group_predecessors[successor as usize] == 0 {
                    ready_group.push(successor as usize);
                }
            }

            let mut group_size = group_items[group].len();

            for item in group_items[group].iter().copied() {
                if item_predecessors[item as usize] == 0 {
                    ready_items.push(item);
                }
            }

            // println!("Ready: {:?}", ready_items);

            while let Some(item) = ready_items.pop() {
                // println!("Item {}: {:?}", item, item_successors[item as usize]);

                remaining_items -= 1;
                group_size -= 1;

                for successor in item_successors[item as usize].iter().copied() {
                    item_predecessors[successor as usize] -= 1;
                    if item_predecessors[successor as usize] == 0 {
                        ready_items.push(successor);
                    }
                }

                res.push(item as i32);
            }

            if group_size > 0 {
                break;
            }
        }

        if remaining_items == 0 && remaining_groups == 0 {
            res
        } else {
            vec![]
        }
    }
}

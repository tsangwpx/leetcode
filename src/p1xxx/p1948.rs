// Problem 1948
impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        /*
         * Trie + toposort + hashmap-based counter + backtracking
         */
        use std::collections::HashMap;
        use std::collections::VecDeque;
        use std::rc::Rc;

        #[derive(Debug)]
        struct Node<'a> {
            parent: usize,
            name: &'a str,
            predecessors: i32,
            children: HashMap<&'a String, usize>,
            unique_index: Option<usize>,
        }

        let mut nodes = vec![Node {
            parent: 0,
            name: &"",
            predecessors: 0,
            children: Default::default(),
            unique_index: None,
        }];

        fn add<'a>(nodes: &mut Vec<Node<'a>>, node_idx: usize, path: &'a [String]) {
            if let Some(part) = path.get(0) {
                let node_len = nodes.len();

                let next_idx = *nodes[node_idx].children.entry(part).or_insert(node_len);
                if next_idx == node_len {
                    let node = Node {
                        parent: node_idx,
                        name: part,
                        predecessors: 0,
                        children: Default::default(),
                        unique_index: None,
                    };
                    nodes.push(node);
                }

                add(nodes, next_idx, &path[1..]);
            } else {
                let parent_idx = nodes[node_idx].parent;
                nodes[parent_idx].predecessors += 1;
            }
        }

        for path in paths.iter() {
            add(&mut nodes, 0, &path);
        }

        // toposort
        let mut pending = nodes
            .iter()
            .enumerate()
            .filter_map(|(idx, node)| {
                if node.predecessors == 0 {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<VecDeque<_>>();

        #[derive(Clone, Debug, Default)]
        struct UniqueTree {
            index: usize,
            unique_children: Rc<Vec<usize>>,
        }

        // A unique tree is a name + a unique children
        // A unique children is a ordered list of unique trees
        // Two unique trees are trimmed if they have the same unique children

        let mut unique_trees = vec![];
        let mut unique_tree_table = HashMap::<(&str, Rc<Vec<usize>>), usize>::new();
        let mut unique_children_counter = HashMap::<Rc<Vec<usize>>, i32>::new();

        while let Some(node_idx) = pending.pop_front() {
            let mut unique_children = nodes[node_idx]
                .children
                .values()
                .map(|&s| nodes[s].unique_index.clone().unwrap())
                .collect::<Vec<_>>();
            unique_children.sort_unstable();

            let mut unique_children = Rc::new(unique_children);

            match unique_children_counter.entry(unique_children.clone()) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    unique_children = entry.key().clone();
                    *entry.get_mut() += 1;
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }

            let key = (nodes[node_idx].name, unique_children.clone());
            let unique_idx = *unique_tree_table.entry(key).or_insert_with(|| {
                let unique_idx = unique_trees.len();
                let ut = UniqueTree {
                    index: unique_idx,
                    unique_children: unique_children.clone(),
                };
                unique_trees.push(ut);
                unique_idx
            });

            nodes[node_idx].unique_index = Some(unique_idx);
            let parent_idx = nodes[node_idx].parent;
            nodes[parent_idx].predecessors -= 1;

            if nodes[parent_idx].predecessors == 0 {
                pending.push_back(parent_idx);
            }
        }

        let mut result = vec![];
        let mut path = vec![];

        fn dfs<'a>(
            result: &mut Vec<Vec<String>>,
            nodes: &Vec<Node<'a>>,
            unique_trees: &Vec<UniqueTree>,
            unique_children_counter: &HashMap<Rc<Vec<usize>>, i32>,
            node_idx: usize,
            path: &mut Vec<&'a String>,
        ) {
            let mut trimmed = false;
            if let Some(unique_idx) = nodes[node_idx].unique_index {
                let unique_children = unique_trees[unique_idx].unique_children.clone();

                if unique_children.len() >= 1
                    && unique_children_counter
                        .get(&unique_children)
                        .copied()
                        .unwrap_or(0)
                        >= 2
                {
                    trimmed = true;
                }
            }

            if trimmed {
                return;
            }

            if !path.is_empty() {
                result.push(path.iter().map(|&s| s.clone()).collect::<Vec<_>>());
            }

            for (&child_name, &child_idx) in nodes[node_idx].children.iter() {
                path.push(child_name);

                dfs(
                    result,
                    nodes,
                    unique_trees,
                    unique_children_counter,
                    child_idx,
                    path,
                );

                path.pop();
            }
        }

        dfs(
            &mut result,
            &nodes,
            &unique_trees,
            &unique_children_counter,
            0,
            &mut path,
        );

        result
    }
}

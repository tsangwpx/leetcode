// Problem 1233
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        struct TrieNode {
            terminal: bool,
            children: HashMap<String, TrieNode>,
        }

        let mut root = TrieNode {
            terminal: false,
            children: HashMap::new(),
        };

        fn insert(node: &mut TrieNode, entry: &str) {
            if node.terminal {
                return;
            }

            let (name, remaining) = if let Some((name, remaining)) = entry.split_once('/') {
                (name, Some(remaining))
            } else {
                (entry, None)
            };

            let child = if let Some(child) = node.children.get_mut(name) {
                child
            } else {
                let child = TrieNode {
                    terminal: false,
                    children: HashMap::new(),
                };

                node.children.insert(name.to_owned(), child);
                node.children.get_mut(name).unwrap()
            };

            if let Some(remaining) = remaining {
                insert(child, remaining);
            } else {
                child.terminal = true;
                child.children.clear();
            }
        }

        for entry in folder {
            // remove the leading slash
            let Some((empty, remaining)) = entry.split_once('/') else {
                unreachable!("No leading slash");
            };
            assert!(empty.is_empty());

            insert(&mut root, remaining);
        }

        fn traverse<'a>(node: &'a TrieNode, stack: &mut Vec<&'a str>, res: &mut Vec<String>) {
            if node.terminal {
                let path = stack.join("/");
                res.push(path);
                return;
            }

            for (name, child) in node.children.iter() {
                stack.push(&name);
                traverse(child, stack, res);
                stack.pop().unwrap();
            }
        }

        let mut res = vec![];
        let mut stack = vec![""];

        traverse(&root, &mut stack, &mut res);

        res
    }
}

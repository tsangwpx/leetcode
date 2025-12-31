// Problem 1233
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;

        let table = folder.iter().map(|s| s.as_str()).collect::<HashSet<_>>();
        let mut res = vec![];

        for entry in folder.iter() {
            let mut offset = 1;
            let mut removed = false;

            while let Some(pos) = entry.bytes().skip(offset).position(|ch| ch == b'/') {
                let (parent, _) = entry.split_at(offset + pos);

                if table.contains(parent) {
                    removed = true;
                    break;
                }

                offset += pos + 1;
            }

            if !removed {
                res.push(entry.to_owned());
            }
        }

        res
    }
}

// Problem 1466
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        // since the graph is also a tree
        // we are safe to turn all non-zero node to a source to zero.

        let mut sources = HashMap::<i32, Vec<i32>>::new();
        let mut sinks = sources.clone();

        for connection in connections {
            let from_city = connection[0] as i32;
            let to_city = connection[1] as i32;
            sources
                .entry(from_city)
                .or_insert_with(|| vec![])
                .push(to_city);

            sinks
                .entry(to_city)
                .or_insert_with(|| vec![])
                .push(from_city);
        }

        let mut changes = 0;
        let mut queue = vec![0i32];
        let mut used = vec![false; n as usize];
        used[0 as usize] = true;

        while let Some(city) = queue.pop() {
            if let Some(sinks) = sinks.get(&city) {
                for &city2 in sinks.iter() {
                    if used[city2 as usize] {
                        continue;
                    }

                    used[city2 as usize] = true;
                    queue.push(city2);
                }
            }

            if let Some(sources) = sources.get(&city) {
                for &city2 in sources.iter() {
                    if used[city2 as usize] {
                        continue;
                    }

                    used[city2 as usize] = true;
                    changes += 1;
                    queue.push(city2);
                }
            }
        }

        changes
    }
}

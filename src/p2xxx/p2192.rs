// Problem 2192
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ancestors = vec![vec![false; n]; n];
        let mut formers = vec![vec![]; n];
        let mut laters = vec![vec![]; n];
        let mut prerequisites = vec![0; n];

        for row in edges.iter() {
            let to = row[1];
            let from = row[0];

            formers[to as usize].push(from);
            laters[from as usize].push(to);
            prerequisites[to as usize] += 1;
        }

        let mut pending = vec![];
        for (idx, count) in prerequisites.iter().copied().enumerate() {
            if count == 0 {
                pending.push(idx);
            }
        }

        while let Some(idx) = pending.pop() {
            for prev in formers[idx as usize].iter().copied() {
                ancestors[idx][prev as usize] = true;

                for pprev in 0..n {
                    ancestors[idx][pprev] |= ancestors[prev as usize][pprev];
                }
            }

            for next in laters[idx as usize].iter().copied() {
                prerequisites[next as usize] -= 1;
                if prerequisites[next as usize] == 0 {
                    pending.push(next as usize);
                }
            }
            // println!("{}: {:?}", idx, ancestors[idx as usize]);
        }

        let res = ancestors
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .enumerate()
                    .filter_map(|(idx, exist)| if exist { Some(idx as i32) } else { None })
                    .collect()
            })
            .collect();

        res
    }
}

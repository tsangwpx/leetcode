impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut inflows = vec![0u16; num_courses as usize];
        // edges: src -> vec![dst]
        let mut edges: Vec<Vec<u16>> = vec![vec![]; num_courses as usize];

        for edge_vec in prerequisites {
            if let Some(&[src, dst]) = edge_vec.get(..2) {
                inflows[dst as usize] += 1;
                edges[src as usize].push(dst as u16);
            }
        }

        let mut pending = inflows
            .iter()
            .enumerate()
            .filter_map(|(i, &s)| if s == 0 { Some(i as u16) } else { None })
            .collect::<Vec<u16>>();

        while let Some(src) = pending.pop() {
            for &dst in edges[src as usize].iter() {
                // remove edge src -> dst
                inflows[dst as usize] -= 1;
                if inflows[dst as usize] == 0 {
                    pending.push(dst);
                }
            }
        }

        inflows.iter().all(|&s| s == 0)
    }
}

struct Solution {}

fn main() {
    println!("Hello World");
}
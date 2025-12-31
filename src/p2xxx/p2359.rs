// Problem 2359
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut dists1 = vec![-1; n];
        let mut dists2 = vec![-1; n];

        let mut cost = 0;
        let mut curr = node1 as usize;

        while dists1[curr] < 0 {
            dists1[curr] = cost;
            cost += 1;

            if edges[curr] < 0 {
                break;
            }

            curr = edges[curr] as usize;
        }

        let mut minimum = i32::MAX;
        let mut min_idx = -1;

        let mut cost = 0;
        let mut curr = node2 as usize;

        while dists2[curr] < 0 {
            dists2[curr] = cost;
            cost += 1;

            if dists1[curr] >= 0 && dists2[curr] >= 0 {
                let max_dist = dists1[curr].max(dists2[curr]);
                if max_dist < minimum {
                    minimum = max_dist;
                    min_idx = curr as i32;
                } else if max_dist == minimum {
                    min_idx = min_idx.min(curr as i32);
                }
            }

            if edges[curr] < 0 {
                break;
            }

            curr = edges[curr] as usize;
        }

        min_idx
    }
}

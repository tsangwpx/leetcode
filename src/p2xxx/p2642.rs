// Problem 2642
struct Graph {
    dists: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut dists = vec![vec![i32::MAX; n]; n];

        for edge in edges {
            let cost = edge[2];
            let to = edge[1] as usize;
            let from = edge[0] as usize;
            dists[from][to] = dists[from][to].min(cost);
        }

        for i in 0..n {
            dists[i][i] = 0;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dists[i][k] == i32::MAX || dists[k][j] == i32::MAX {
                        continue;
                    }

                    let dist = dists[i][j];
                    let alt = dists[i][k] + dists[k][j];

                    if alt < dist {
                        dists[i][j] = alt;
                    }
                }
            }
        }

        Self { dists }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        assert!(edge.len() >= 3);

        let cost = edge[2];
        let to = edge[1] as usize;
        let from = edge[0] as usize;

        if cost >= self.dists[from][to] {
            return;
        }

        self.dists[from][to] = cost;

        let n = self.dists.len();

        let mut visited = vec![false; n];
        visited[to] = true;
        visited[from] = true;

        let mut pending = vec![to, from];

        while let Some(k) = pending.pop() {
            for i in 0..n {
                for j in 0..n {
                    if self.dists[i][k] == i32::MAX || self.dists[k][j] == i32::MAX {
                        continue;
                    }
                    let dist = self.dists[i][j];
                    let alt = self.dists[i][k] + self.dists[k][j];

                    if alt >= dist {
                        continue;
                    }

                    self.dists[i][j] = alt;

                    if !visited[i] {
                        visited[i] = true;
                        pending.push(i);
                    }

                    if !visited[j] {
                        visited[j] = true;
                        pending.push(j);
                    }
                }
            }
        }
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        match self.dists[node1 as usize][node2 as usize] {
            i32::MAX => -1,
            distance @ _ => distance,
        }
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */
fn f() {}

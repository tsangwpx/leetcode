// Problem 2699
impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let mut graph = vec![HashMap::new(); n as usize];

        for edge in edges.iter() {
            let (u, v, w) = (edge[0], edge[1], edge[2]);

            if w >= 1 {
                graph[u as usize].insert(v, w);
                graph[v as usize].insert(u, w);
            }
        }

        const WEIGHT_ONE: i32 = 0;
        const WEIGHT_INFINITY: i32 = i32::MAX;

        fn dijkstra(n: i32, graph: &Vec<HashMap<i32, i32>>, src: i32, dst: i32) -> i32 {
            let mut distances = vec![i32::MAX; n as usize];
            let mut heap = BinaryHeap::new();
            heap.push((Reverse(0), src));
            distances[src as usize] = 0;

            while let Some((Reverse(dist), node)) = heap.pop() {
                if node == dst {
                    return dist;
                }

                if dist > distances[node as usize] {
                    continue;
                }

                for (&neighbor, &weight) in graph[node as usize].iter() {
                    let mut weight = weight;

                    let neighbor_dist = dist + weight;
                    if neighbor_dist < distances[neighbor as usize] {
                        distances[neighbor as usize] = neighbor_dist;
                        heap.push((Reverse(neighbor_dist), neighbor));
                    }
                }
            }

            i32::MAX
        }

        let shortest = dijkstra(n, &graph, source, destination);

        fn remove_unused_edges(edges: &mut [Vec<i32>], value: i32) {
            for edge in edges.iter_mut() {
                let w = &mut edge[2];

                if *w == -1 {
                    *w = value;
                }
            }
        }

        match shortest.cmp(&target) {
            Ordering::Less => vec![],
            Ordering::Equal => {
                remove_unused_edges(&mut edges, target);
                edges
            }
            Ordering::Greater => {
                for edge in edges.iter_mut() {
                    let u = edge[0];
                    let v = edge[1];
                    let w = &mut edge[2];

                    if *w != -1 {
                        continue;
                    }

                    *w = 1;

                    graph[u as usize].insert(v, 1);
                    graph[v as usize].insert(u, 1);

                    let distance = dijkstra(n, &graph, source, destination);

                    if distance <= target {
                        *w += target - distance;
                        remove_unused_edges(&mut edges, target);
                        return edges;
                    }
                }

                vec![]
            }
        }
    }
}

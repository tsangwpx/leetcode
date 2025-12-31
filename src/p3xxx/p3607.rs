// Problem 3607
impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // union find...

        let mut grids = (0..c).collect::<Vec<i32>>();

        #[inline]
        fn find(grids: &mut Vec<i32>, idx: i32) -> i32 {
            let mut parent = grids[idx as usize];

            if parent != grids[parent as usize] {
                parent = find(grids, parent);
                grids[idx as usize] = parent;
            }

            parent
        }

        #[inline]
        fn union(grids: &mut Vec<i32>, idx: i32, idx2: i32) -> i32 {
            let idx = find(grids, idx);
            let idx2 = find(grids, idx2);
            let (idx, idx2) = (idx.min(idx2), idx.max(idx2));
            grids[idx2 as usize] = idx;
            idx
        }

        for conn in connections.into_iter() {
            let &[u, v] = conn.as_slice() else {
                panic!("bad connection tuple");
            };

            union(&mut grids, u - 1, v - 1);
        }

        let mut station_online = vec![true; c as usize];
        let mut stations_by_grid = vec![vec![]; c as usize];

        for station in (0..c).rev() {
            let grid = find(&mut grids, station);
            stations_by_grid[grid as usize].push(station);
        }

        let mut res = vec![];

        for query in queries.into_iter() {
            match query.as_slice() {
                &[1, station] => {
                    let station = station - 1;

                    let mut result = -1;

                    if station_online[station as usize] {
                        result = station + 1
                    } else {
                        let grid = find(&mut grids, station);

                        while let Some(smallest) = stations_by_grid[grid as usize].last().copied() {
                            if station_online[smallest as usize] {
                                result = smallest + 1;
                                break;
                            } else {
                                stations_by_grid[grid as usize].pop();
                            }
                        }
                    };

                    res.push(result);
                }

                &[2, station] => {
                    let station = station - 1;
                    station_online[station as usize] = false;
                }
                _ => {
                    panic!("Bad query")
                }
            }
        }

        res
    }
}

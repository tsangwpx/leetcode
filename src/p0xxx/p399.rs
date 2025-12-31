// Problem 399
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // Dijkstra's algorithm

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        use std::rc::Rc;

        let mut vertices = HashMap::<Rc<String>, u8>::with_capacity(equations.len() * 2);
        let mut edges = vec![HashMap::<u8, f64>::new(); 0];

        fn intern<T>(s: String, table: &HashMap<Rc<String>, T>) -> Rc<String> {
            // string interning
            let rc = Rc::new(s);
            table
                .get_key_value(&rc)
                .map_or_else(|| rc, |(s, _)| s.clone())
        }

        for (mut vars, value) in equations.into_iter().zip(values.into_iter()) {
            // create edge table for each new variable

            let den = intern(vars.pop().unwrap(), &vertices);
            let new_idx = vertices.len() as u8;
            let den_idx = *vertices.entry(den).or_insert_with(|| {
                edges.push(Default::default());
                new_idx
            });

            let num = intern(vars.pop().unwrap(), &vertices);
            let new_idx = vertices.len() as u8;
            let num_idx = *vertices.entry(num).or_insert_with(|| {
                edges.push(Default::default());
                new_idx
            });

            // insert the edge value
            edges[num_idx as usize].insert(den_idx, value);
            edges[den_idx as usize].insert(num_idx, 1.0 / value);
        }

        // result vec
        let mut res = Vec::with_capacity(queries.len());

        // heap for Dijkstra's algorithm
        // first item is distance, which constantly increase one by one each examination
        // second item is the node index
        let mut heap = BinaryHeap::<(Reverse<u8>, u8)>::with_capacity(edges.len() * 2);

        // this vec holds the latest node dist and query value
        let mut values = vec![(u8::MAX, -1.0); edges.len()];

        for mut vars in queries {
            let den = vars.pop().unwrap();
            let num = vars.pop().unwrap();

            let num_idx = vertices.get(&num);
            let den_idx = vertices.get(&den);

            if den_idx.is_none() || num_idx.is_none() {
                // variable is not recognized
                res.push(-1.0);
                continue;
            }

            let den_idx = *den_idx.unwrap();
            let num_idx = *num_idx.unwrap();

            if den_idx == num_idx {
                // sepecial case
                res.push(1.0);
                continue;
            }

            // reset values and heap
            values.fill((u8::MAX, -1.0));
            values[num_idx as usize] = (0u8, 1.0);

            heap.clear();
            heap.extend((0..edges.len()).map(|s| (Reverse(values[s].0), s as u8)));

            'outer: while let Some((Reverse(d), u_idx)) = heap.pop() {
                let (u_dist, xu_value) = values[u_idx as usize];

                if u_dist == u8::MAX || u_dist != d {
                    // this node is too far aways or is outdated
                    continue;
                }

                for (&v_idx, &uv_value) in edges[u_idx as usize].iter() {
                    let (v_dist, _) = values[v_idx as usize];

                    if u_dist + 1 < v_dist {
                        // u can reach v
                        // the query value is given by x/u * u/v
                        let value = xu_value * uv_value;
                        values[v_idx as usize] = (u_dist + 1, value);

                        if v_idx == den_idx {
                            // break if reach dest
                            break 'outer;
                        }

                        heap.push((Reverse(u_dist + 1), v_idx));
                    }
                }
            }

            res.push(values[den_idx as usize].1);
        }

        res
    }

    pub fn calc_equation2(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // Floyd–Warshall algorithm
        use std::collections::HashMap;

        let mut vertices = HashMap::<&str, u8>::with_capacity(equations.len() * 2);
        let mut map = vec![vec![-1.0; 41]; 41];

        for (vars, value) in equations.iter().zip(values.iter()) {
            let num = &vars[0];
            let den = &vars[1];

            let new_idx = vertices.len() as u8;
            let num_idx = *vertices.entry(num).or_insert_with(|| new_idx);

            let new_idx = vertices.len() as u8;
            let den_idx = *vertices.entry(den).or_insert_with(|| new_idx);

            map[num_idx as usize][den_idx as usize] = *value;

            if num_idx != den_idx {
                map[den_idx as usize][num_idx as usize] = 1.0 / *value;

                map[num_idx as usize][num_idx as usize] = 1.0;
                map[den_idx as usize][den_idx as usize] = 1.0;
            }
        }

        let mut res = Vec::with_capacity(queries.len());
        let mut len = vertices.len();
        assert!(len < map.len());

        for k in 0..len {
            for i in 0..len {
                for j in 0..len {
                    let vik = if map[i][k] >= 0.0 {
                        map[i][k]
                    } else {
                        1.0 / map[k][i]
                    };

                    let vkj = if map[k][j] >= 0.0 {
                        map[k][j]
                    } else {
                        1.0 / map[j][k]
                    };
                    if vik >= 0.0 && vkj >= 0.0 {
                        let value = vik * vkj;
                        map[i][j] = value;
                        map[j][i] = 1.0 / value;
                    }
                }
            }
        }

        // println!("{:?}", vertices);

        // for i in 0..len {
        //     println!("{}: {:?}", i, &map[i][..len]);
        // }

        for vars in queries.iter() {
            let num = &vars[0];
            let den = &vars[1];

            let num_idx = vertices.get(num.as_str());
            let den_idx = vertices.get(den.as_str());

            if den_idx.is_none() || num_idx.is_none() {
                res.push(-1.0);
                continue;
            }

            let den_idx = *den_idx.unwrap();
            let num_idx = *num_idx.unwrap();

            res.push(map[num_idx as usize][den_idx as usize]);
        }

        res
    }
}

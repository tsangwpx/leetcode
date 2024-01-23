mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 815
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        use std::collections::HashSet;

        let route_count = routes.len();

        let mut stop2routes = HashMap::new();

        for (route_id, route) in routes.iter().enumerate() {
            let route_id = route_id as u16;

            for stop in route.iter().copied() {
                stop2routes
                    .entry(stop)
                    .or_insert_with(|| Vec::with_capacity(route_count))
                    .push(route_id);
            }
        }

        let mut heap = BinaryHeap::new();
        let mut route_taken = vec![false; route_count];

        if let Some(routes) = stop2routes.get(&source) {
            for route_id in routes.iter().copied() {
                heap.push((Reverse(1i16), route_id));
                route_taken[route_id as usize] = true;
            }
        }

        while let Some((Reverse(bus_taken), route_id)) = heap.pop() {
            for stop in routes[route_id as usize].iter().copied() {
                if stop == target {
                    return bus_taken as i32;
                }

                let Some(exchange_routes) = stop2routes.get(&stop) else {
                    continue;
                };

                for exchange_id in exchange_routes.iter().copied() {
                    if route_taken[exchange_id as usize] {
                        continue;
                    }

                    route_taken[exchange_id as usize] = true;
                    heap.push((Reverse(bus_taken + 1), exchange_id));
                }
            }
        }

        -1
    }
    pub fn num_buses_to_destination2(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        use std::collections::HashSet;

        let route_count = routes.len();
        let mut stops = HashMap::new();

        for (route_id, route) in routes.into_iter().enumerate() {
            let route_id = route_id as u16;

            for stop in route {
                stops.entry(stop).or_insert_with(|| vec![]).push(route_id);
            }
        }

        let Some(source_hub) = stops.remove(&source) else {
            return -1;
        };
        let Some(target_hub) = stops.remove(&target) else {
            return -1;
        };

        stops.retain(|_, routes| routes.len() >= 2);

        let mut unique_hubs = stops.into_values().collect::<HashSet<Vec<u16>>>();
        unique_hubs.remove(&source_hub);
        unique_hubs.remove(&target_hub);

        let node_count = 2 + route_count + unique_hubs.iter().map(|s| s.len()).sum::<usize>();
        let mut dists = vec![i16::MAX; node_count];
        let mut friends = vec![vec![(0u32, 0i16); 0]; node_count];

        dists[0] = 0;

        for bus in source_hub.iter().copied() {
            let bus_node = bus as u32 + 2;
            friends[0].push((bus_node, 0i16));
        }

        for bus in target_hub.iter().copied() {
            let bus_node = bus as usize + 2;
            friends[bus_node].push((1, 0i16));
        }

        let mut idx = 2 + route_count;

        for hub in unique_hubs.into_iter() {
            let start = idx;
            let stop = idx + hub.len();

            for i in start..stop {
                for j in i + 1..stop {
                    friends[i].push((j as u32, 1));
                    friends[j].push((i as u32, 1));
                }
            }

            for (i, &bus) in hub.iter().enumerate() {
                let bus_node = bus as u32 + 2;
                let hub_node = (start + i) as u32;

                friends[bus_node as usize].push((hub_node, 0));
                friends[hub_node as usize].push((bus_node, 0));
            }

            idx += hub.len();
        }

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0i16), 0u32));

        while let Some((Reverse(dist), node_id)) = heap.pop() {
            if dists[node_id as usize] != dist {
                continue;
            }

            for (friend_node, cost) in friends[node_id as usize].iter().copied() {
                let new_dist = dist + cost;
                if new_dist < dists[friend_node as usize] {
                    if friend_node == 1 {
                        return new_dist as i32 + 1;
                    }

                    dists[friend_node as usize] = new_dist;
                    heap.push((Reverse(new_dist), friend_node));
                }
            }
        }

        if dists[1] == i16::MAX {
            -1
        } else {
            dists[1] as i32 + 1
        }
    }
}

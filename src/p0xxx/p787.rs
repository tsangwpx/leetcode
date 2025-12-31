// Problem 787
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::collections::VecDeque;

        let n = n as usize;
        let k = k as usize;
        let src = src as usize;
        let dst = dst as usize;
        assert!(k < n);

        let mut table = vec![vec![]; n];

        for flight in flights {
            let price = flight[2];
            let dst = flight[1] as usize;
            let src = flight[0] as usize;
            if dst < n && src < n {
                table[src].push((dst, price));
            }
        }

        // Prices from src to another city
        let mut cheapest_costs = vec![i32::MAX; n];
        cheapest_costs[src] = 0;
        let mut snapshot = Vec::with_capacity(cheapest_costs.len());

        let mut origins = VecDeque::with_capacity(n);
        origins.push_back(src);
        // println!("{:?} {:?}", cheapest_costs, origins);

        // k stops => k + 1 tickets/flights
        for _ in 0..k + 1 {
            snapshot.clone_from(&cheapest_costs);

            let len = origins.len();

            for _ in 0..len {
                let from = origins.pop_front().unwrap();
                for (to, price) in table[from].iter().copied() {
                    let cost = snapshot[from] + price;

                    if cost >= cheapest_costs[to] {
                        continue;
                    }

                    cheapest_costs[to] = cost;
                    origins.push_back(to);
                }
            }

            // println!("{:?} {:?}", cheapest_costs, origins);

            if origins.is_empty() {
                break;
            }
        }

        if cheapest_costs[dst] == i32::MAX {
            -1
        } else {
            cheapest_costs[dst]
        }
    }
}

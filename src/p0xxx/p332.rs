mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 332
impl Solution {
    pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let len = tickets.len();
        let mut table = HashMap::new();

        for flight in tickets.iter_mut() {
            assert!(flight.len() >= 2);
            let stop = flight.pop().unwrap();
            let start = flight.pop().unwrap();
            table.entry(start).or_insert_with(|| vec![]).push(stop);
        }

        table
            .values_mut()
            .for_each(|s| s.sort_unstable_by(|a, b| a.cmp(b).reverse()));

        let mut stack = vec!["JFK".to_owned()];
        stack.reserve(len);
        let mut ltinerary = Vec::with_capacity(len + 1);

        while let Some(top) = stack.pop() {
            let next = {
                let mut next = None;
                let mut len = 0;
                if let Some(queue) = table.get_mut(&top) {
                    next = queue.pop();
                    len = queue.len();
                }

                if len == 0 {
                    table.remove(&top);
                }

                next
            };

            if let Some(next) = next {
                stack.push(top);
                stack.push(next);
            } else {
                ltinerary.push(top);
            }
        }

        ltinerary.reverse();
        ltinerary
    }
}

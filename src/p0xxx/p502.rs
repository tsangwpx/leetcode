mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 502
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        assert!(profits.len() == capital.len());
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut money = w;
        let mut heap = Vec::<i32>::with_capacity(profits.len());
        let mut projects = Vec::<(Reverse<i32>, i32)>::with_capacity(profits.len());

        projects.extend(profits.into_iter().zip(capital.into_iter()).filter_map(
            |(profit, capital)| {
                if profit <= 0 {
                    // remove project without profit
                    None
                } else if capital <= money {
                    heap.push(profit);
                    None
                } else {
                    Some((Reverse(capital), profit))
                }
            },
        ));

        let mut projects = BinaryHeap::from(projects);
        let mut heap = BinaryHeap::from(heap);

        // println!("{:?}", projects);
        // println!("{:?}", heap);

        for _ in 0..k {
            // add projects with fulfilled capital to heap.
            while let Some(&(Reverse(capital), profit)) = projects.peek() {
                if money >= capital {
                    projects.pop();
                    heap.push(profit);
                } else {
                    break;
                }
            }

            // pop the project with the max profit.
            if let Some(profit) = heap.pop() {
                money += profit;
            }
        }

        money
    }

    pub fn find_maximized_capital2(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        // implement with one sorted list + one heap seem faster than two heaps

        use std::collections::BinaryHeap;

        let mut projects = profits
            .into_iter()
            .zip(capital.into_iter())
            .filter_map(|(profit, capital)| {
                if profit > 0 {
                    Some((capital, profit))
                } else {
                    // remove project without profit
                    None
                }
            })
            .collect::<Vec<_>>();

        projects.sort_unstable();

        let projects = projects;

        let mut heap = BinaryHeap::<i32>::with_capacity(projects.len());
        let mut money = w;
        let mut idx = 0;

        for _ in 0..k {
            // add projects with fulfilled capital to heap.
            while idx < projects.len() && projects[idx].0 <= money {
                heap.push(projects[idx].1);
                idx += 1;
            }

            // pop the project with the max profit.
            if let Some(profit) = heap.pop() {
                money += profit;
            }
        }

        money
    }
}

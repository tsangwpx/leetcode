// Problem 2462
impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let candidates = candidates as usize;

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        struct Worker {
            cost: Reverse<i32>,
            index: Reverse<usize>,
            right: bool,
        }

        impl Worker {
            fn new(cost: i32, index: usize, right: bool) -> Self {
                Self {
                    cost: Reverse(cost),
                    index: Reverse(index),
                    right,
                }
            }
        }

        let mut queue = BinaryHeap::<Worker>::with_capacity(candidates * 2);
        let mut start = 0;
        let mut stop = costs.len();
        let mut total_cost = 0i64;

        // Fill some candidates from the front
        for _ in 0..candidates {
            if start >= stop {
                break;
            }

            queue.push(Worker::new(costs[start], start, false));
            start += 1;
        }

        // Fill some candidates from the back
        for _ in 0..candidates {
            if start >= stop {
                break;
            }
            stop -= 1;
            queue.push(Worker::new(costs[stop], stop, true));
        }

        for _ in 0..k {
            if let Some(worker) = queue.pop() {
                // println!("{:?}", worker);
                total_cost += worker.cost.0 as i64;

                if start < stop {
                    // workers are available to refill the queue

                    if worker.right {
                        // pick one from right
                        stop -= 1;
                        queue.push(Worker::new(costs[stop], stop, true));
                    } else {
                        // pick one from left
                        queue.push(Worker::new(costs[start], start, false));
                        start += 1;
                    }
                }
            }
        }

        total_cost
    }
}

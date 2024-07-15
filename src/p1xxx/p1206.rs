mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

extern crate rand;

// Problem 1206
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Debug, Clone, Copy)]
struct Node {
    value: i32,
    next: usize,
    down: usize,
}

#[derive(Debug)]
struct Skiplist {
    free: Vec<usize>,
    nodes: Vec<Node>,
    head: usize,
    dist: Bernoulli,
    rng: StdRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    const DUMMY_VALUE: i32 = -1;
    const NULL_NODE: usize = usize::MAX;

    fn new() -> Self {
        Self {
            free: vec![],
            nodes: vec![Node {
                value: Self::DUMMY_VALUE,
                next: Self::NULL_NODE,
                down: Self::NULL_NODE,
            }],
            head: 0,
            dist: Bernoulli::new(0.5).unwrap(),
            rng: StdRng::from_seed([0u8; 32]),
        }
    }

    fn search(&self, target: i32) -> bool {
        let mut pivot = self.head;

        while pivot != Self::NULL_NODE {
            let Node { value, next, down } = self.nodes[pivot];

            if value == target {
                return true;
            }

            if next != Self::NULL_NODE && target >= self.nodes[next].value {
                pivot = next;
            } else {
                pivot = down;
            }
        }

        false
    }

    fn _locate(&self, num: i32) -> Vec<usize> {
        let mut stack = vec![];

        let mut pivot = self.head;

        while pivot != Self::NULL_NODE {
            let Node { next, down, .. } = self.nodes[pivot];

            if next == Self::NULL_NODE || self.nodes[next].value >= num {
                stack.push(pivot);
                pivot = down;
            } else {
                pivot = next;
            }
        }

        stack
    }

    fn add(&mut self, num: i32) {
        let mut stack = self._locate(num);

        let mut insert = true;
        let mut down = Self::NULL_NODE;

        while let Some(pivot) = stack.pop() {
            let new_node = self._add_node(Node {
                value: num,
                next: self.nodes[pivot].next,
                down,
            });
            self.nodes[pivot].next = new_node;
            down = new_node;

            insert = self.dist.sample(&mut self.rng);
            if !insert {
                break;
            }
        }

        if insert {
            // The first level must be the single dummy node.
            let new_head = self._add_node(Node {
                value: Self::DUMMY_VALUE,
                next: Self::NULL_NODE,
                down: self.head,
            });
            self.head = new_head;
        }
    }

    fn _add_node(&mut self, node: Node) -> usize {
        if let Some(idx) = self.free.pop() {
            self.nodes[idx] = node;
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(node);
            idx
        }
    }

    fn erase(&mut self, num: i32) -> bool {
        let mut stack = self._locate(num);
        let mut found = false;

        for pivot in stack.iter().rev().copied() {
            let next = self.nodes[pivot].next;
            if next == Self::NULL_NODE || self.nodes[next].value != num {
                break;
            }
            found = true;
            self.nodes[pivot].next = self.nodes[next].next;
            self._erase_node(next);
        }

        let head = self.head;

        for lower in stack.iter().skip(1).copied() {
            let Node { value, next, down } = self.nodes[lower];

            if value == Self::DUMMY_VALUE && next == Self::NULL_NODE {
                self.nodes[head].down = down;
                self._erase_node(lower);
            } else {
                break;
            }
        }

        found
    }

    fn _erase_node(&mut self, idx: usize) {
        self.free.push(idx);
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */
fn dummy() {}

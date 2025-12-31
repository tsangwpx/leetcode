// Problem 432
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::rc::Rc;

#[derive(Debug, Default)]
struct Node {
    count: i32,
    members: HashSet<Rc<String>>,
    prev: usize,
    next: usize,
}

#[derive(Default, Debug)]
struct AllOne {
    counter: HashMap<Rc<String>, i32>,
    count2nodes: HashMap<i32, usize>,
    nodes: Vec<Node>,
    unused: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    const NULL_NODE: usize = usize::MAX;
    const DUMMY_HEAD: usize = 0;
    const DUMMY_TAIL: usize = 1;

    fn new() -> Self {
        let mut inst = Self::default();

        // Dummy head
        inst.nodes.push(Node {
            count: 0,
            members: HashSet::default(),
            prev: Self::NULL_NODE,
            next: Self::DUMMY_TAIL,
        });
        inst.count2nodes.insert(0, Self::DUMMY_HEAD);

        // Dummy tail
        inst.nodes.push(Node {
            count: i32::MAX,
            members: HashSet::default(),
            prev: Self::DUMMY_HEAD,
            next: Self::NULL_NODE,
        }); // dummy tail node
        inst.count2nodes.insert(i32::MAX, Self::DUMMY_TAIL);

        inst
    }

    fn _get_next_node(&mut self, node: usize) -> usize {
        let Node { count, next, .. } = self.nodes[node];
        let expected_count = count + 1;

        if self.nodes[next].count == expected_count {
            // next node is correct
            next
        } else {
            // next become nextnext
            let nextnext = next;
            let next = self.unused.pop().unwrap_or_else(|| {
                let idx = self.nodes.len();
                self.nodes.push(Node::default());
                idx
            });

            self.count2nodes.insert(expected_count, next);

            self.nodes[next].count = expected_count;
            self.nodes[next].prev = node;
            self.nodes[next].next = nextnext;

            self.nodes[node].next = next;
            self.nodes[nextnext].prev = next;

            next
        }
    }

    fn _get_prev_node(&mut self, node: usize) -> usize {
        let Node { count, prev, .. } = self.nodes[node];
        assert!(count >= 1);
        let expected_count = count - 1;

        if self.nodes[prev].count == expected_count {
            prev
        } else {
            let prevprev = prev;
            let prev = self.unused.pop().unwrap_or_else(|| {
                let idx = self.nodes.len();
                self.nodes.push(Node::default());
                idx
            });
            self.count2nodes.insert(expected_count, prev);

            self.nodes[prev].count = expected_count;
            self.nodes[prev].prev = prevprev;
            self.nodes[prev].next = node;

            self.nodes[node].prev = prev;
            self.nodes[prevprev].next = prev;

            prev
        }
    }

    fn _unlink_if_empty(&mut self, node: usize) {
        if node == Self::DUMMY_HEAD || node == Self::DUMMY_TAIL {
            // Dont remove these two nodes
            return;
        }

        // println!("unlink if empty: {}={:?}", node, self.nodes[node]);

        if self.nodes[node].members.is_empty() {
            let Node {
                count, prev, next, ..
            } = self.nodes[node];
            self.nodes[prev].next = next;
            self.nodes[next].prev = prev;
            self.count2nodes.remove(&count).unwrap();
            self.unused.push(node);
        }
    }

    fn inc(&mut self, key: String) {
        let (key, new_count) = match self.counter.entry(Rc::new(key)) {
            Occupied(mut entry) => {
                let key = Rc::clone(entry.key());
                let count = entry.get_mut();
                *count += 1;
                (key, *count)
            }
            Vacant(entry) => {
                let key = Rc::clone(entry.key());
                (key, *entry.insert(1))
            }
        };
        // println!("inc {} {}", key, new_count);

        let old_count = new_count - 1;
        let old_node = self.count2nodes.get(&old_count).copied().unwrap();

        let new_node = self._get_next_node(old_node);

        self.nodes[old_node].members.remove(&key);
        self.nodes[new_node].members.insert(key);

        self._unlink_if_empty(old_node);
        // println!(
        //     "{:?} {:?} {:?} {:?}",
        //     self.counter, self.nodes, self.count2nodes, self.unused
        // );
    }

    fn dec(&mut self, key: String) {
        let (key, new_count) = match self.counter.entry(Rc::new(key)) {
            Occupied(mut entry) => {
                let count = entry.get_mut();
                *count -= 1;

                if *count == 0 {
                    entry.remove_entry()
                } else {
                    let count = *count;
                    (Rc::clone(entry.key()), count)
                }
            }
            Vacant(entry) => {
                return;
            }
        };
        // println!("dec {} {}", key, new_count);

        let old_count = new_count + 1;
        let old_node = self.count2nodes.get(&old_count).copied().unwrap();

        let new_node = self._get_prev_node(old_node);

        self.nodes[old_node].members.remove(&key);
        if new_count >= 1 {
            self.nodes[new_node].members.insert(key);
        }

        self._unlink_if_empty(old_node);
        // println!(
        //     "{:?} {:?} {:?} {:?}",
        //     self.counter, self.nodes, self.count2nodes, self.unused
        // );
    }

    fn get_max_key(&mut self) -> String {
        let tail = self.nodes[Self::DUMMY_TAIL].prev;

        if tail == Self::DUMMY_HEAD {
            return "".to_owned();
        }

        String::clone(self.nodes[tail].members.iter().next().unwrap())
    }

    fn get_min_key(&mut self) -> String {
        let head = self.nodes[Self::DUMMY_HEAD].next;

        if head == Self::DUMMY_TAIL {
            "".to_owned()
        } else {
            String::clone(self.nodes[head].members.iter().next().unwrap())
        }
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
fn _f() {}

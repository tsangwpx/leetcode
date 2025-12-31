// Problem 3508
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Source = i32;
type Destination = i32;
type Timestamp = i32;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Triplet(Source, Destination, Timestamp);

struct Router {
    limit: usize,
    deque: VecDeque<Triplet>,
    unique: HashSet<Triplet>,
    dest_packets: HashMap<i32, VecDeque<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memoryLimit: i32) -> Self {
        let limit = memoryLimit as usize;

        Self {
            limit,
            deque: VecDeque::with_capacity(limit),
            unique: HashSet::with_capacity(limit),
            dest_packets: Default::default(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let triplet = Triplet(source, destination, timestamp);

        if !self.unique.insert(triplet) {
            return false;
        }

        if self.deque.len() == self.limit {
            self.forward_packet();
        }

        self.deque.push_back(triplet);
        self.dest_packets
            .entry(destination)
            .or_default()
            .push_back(timestamp);

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(triplet) = self.deque.pop_front() {
            let Triplet(_, dest, _) = triplet;

            self.unique.remove(&triplet);
            match self.dest_packets.entry(dest) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    if entry.get().len() == 1 {
                        entry.remove();
                    } else {
                        entry.get_mut().pop_front();
                    }
                }
                std::collections::hash_map::Entry::Vacant(_entry) => panic!("entry gone"),
            }

            let Triplet(src, dst, time) = triplet;
            vec![src, dst, time]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(deque) = self.dest_packets.get(&destination) {
            let start = deque.partition_point(|&s| s < start_time);
            let end = deque.partition_point(|&s| s <= end_time);

            (end - start) as i32
        } else {
            0
        }
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
fn _d() {}

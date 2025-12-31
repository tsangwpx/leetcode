// Problem 731
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Default, Debug)]
struct MyCalendarTwo {
    counter: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        // From others' solutions,
        // the idea is keep a counter of the ongoing meetings
        // when enter an interval, the counter increase
        // when exit an interval, the counter decrease
        // Since the maximum number of ongoing meetings is 2,
        // the balance can't be 3 or the booking is invalid.
        // The algorithm below first add the booking to the system,
        // and iterate all points in time to verify that no more than 2 bookings overlaps.
        // If something go wrong, undo the booking.

        *self.counter.entry(start).or_default() += 1;
        *self.counter.entry(end).or_default() -= 1;

        let mut balance = 0;

        for (_, &count) in self.counter.iter() {
            balance += count;

            if balance >= 3 {
                *self.counter.get_mut(&start).unwrap() -= 1;
                *self.counter.get_mut(&end).unwrap() += 1;
                return false;
            }
        }
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
fn _unused() {}

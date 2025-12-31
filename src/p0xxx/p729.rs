// Problem 729
use std::collections::BTreeSet;

#[derive(Debug, Default)]
struct MyCalendar {
    events: BTreeSet<(i32, i8)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        use std::ops::Bound::{Included, Unbounded};

        // At a particular time, START is after END
        const START: i8 = 1;
        const END: i8 = 0;

        if let Some((time, side)) = self
            .events
            .range((Included(&(start, START)), Unbounded))
            .next()
            .copied()
        {
            if time < end {
                // There is another event start or end strictly before our end
                return false;
            } else if time >= end && side == END {
                // Our interval is contained in another event
                return false;
            }
        }

        self.events.insert((start, START));
        self.events.insert((end, END));
        // println!("{} {}: {:?}", start, end, self.events);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
fn _unused() {}

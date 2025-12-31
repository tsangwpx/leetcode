// Problem 1942
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let target_arrival = times[target_friend as usize][0];

        let mut events = BinaryHeap::with_capacity(times.len());

        for (friend, row) in times.iter().enumerate() {
            let arrival = row[0];

            if arrival > target_arrival {
                continue;
            }

            events.push((Reverse(arrival), -1, friend as i32));
        }

        let mut empty_seats = BinaryHeap::new();
        let mut seat_allocated = 0;

        while let Some((_, seat, friend)) = events.pop() {
            if seat < 0 {
                // negative seat number is arrival

                let seat = if let Some(Reverse(seat)) = empty_seats.pop() {
                    seat
                } else {
                    let seat = seat_allocated;
                    seat_allocated += 1;
                    seat
                };

                if friend == target_friend {
                    return seat;
                }

                let leaving = times[friend as usize][1];
                events.push((Reverse(leaving), seat, friend));
            } else {
                // non-negative seat number is leaving
                empty_seats.push(Reverse(seat));
            }
        }

        unreachable!()
    }
}

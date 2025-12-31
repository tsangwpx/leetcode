// Problem 2402
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::iter::FromIterator;

        meetings.sort_by_key(|s| s[0]);

        // unused rooms
        let mut unused = BinaryHeap::from_iter((0..n).map(Reverse));

        // meetings in progress, value: Reverse((END_TIME, ROOM_ID))
        let mut running = BinaryHeap::<Reverse<(i64, i32)>>::new();
        let mut counter = vec![0; n as usize];

        for meeting in meetings.iter() {
            let &[start, end] = meeting.as_slice() else {
                panic!("bad format");
            };

            let duration = (end - start) as i64;
            let start = start as i64;

            while let Some(Reverse((end2, room))) = running.peek().copied() {
                // Complete a meeting if time is passed, or we need an unused room
                if end2 > start {
                    break;
                }

                running.pop();
                unused.push(Reverse(room));
            }

            if let Some(Reverse(room)) = unused.pop() {
                running.push(Reverse((end as i64, room)));
                counter[room as usize] += 1;
            } else {
                let mut top = running.peek_mut().unwrap();
                let Reverse((finish_time, room)) = &mut *top;
                *finish_time += duration;
                let room = *room;
                counter[room as usize] += 1;
            }
        }

        (0..n).max_by_key(|&i| (counter[i as usize], -i)).unwrap()
    }
}

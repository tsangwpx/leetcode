// Problem 3394
impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        #[derive(Debug, Clone, Copy)]
        enum Direction {
            Horizontal,
            Vertical,
        }

        fn check_direction(n: i32, rectangles: &Vec<Vec<i32>>, direction: Direction) -> bool {
            use std::collections::HashMap;

            let offset = match direction {
                Direction::Horizontal => 0,
                Direction::Vertical => 1,
            };

            #[derive(Debug, Clone, Copy, Default)]
            struct Counter {
                open: i32,
                close: i32,
            }

            let mut events = HashMap::<i32, Counter>::with_capacity(rectangles.len());

            for row in rectangles.iter() {
                if row.len() > offset + 2 {
                    let start = row[offset];
                    let end = row[offset + 2];

                    events.entry(start).or_default().open += 1;
                    events.entry(end).or_default().close += 1;
                }
            }

            let mut events = events
                .into_iter()
                .map(|(pos, Counter { open, close })| (pos, open, close))
                .collect::<Vec<_>>();

            events.sort_unstable();

            let mut overlapping = 0;
            let mut count = 0;
            let mut last_pos = 0;

            for (pos, open, close) in events.iter().cloned() {
                overlapping -= close;

                if open > 0 && overlapping == 0 && last_pos < pos {
                    count += 1;
                    last_pos = pos;
                }

                overlapping += open;
            }

            count >= 2
        }

        check_direction(n, &rectangles, Direction::Horizontal)
            || check_direction(n, &rectangles, Direction::Vertical)
    }
}

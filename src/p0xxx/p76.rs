// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 76
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        const COUNTER_SIZE: usize = 128;

        let counter = t
            .bytes() //
            .into_iter()
            .fold([0; COUNTER_SIZE], |mut counter, ch| {
                counter[ch as usize] += 1;
                counter
            });

        let mut running = [0; COUNTER_SIZE];
        let mut left = 0;
        let mut window: Option<&str> = Option::None;

        for (pos, ch) in s.bytes().into_iter().enumerate() {
            let ch_idx = ch as usize;
            running[ch_idx] += 1;

            let mut left_changed = false;

            // try to increase left
            while left < pos {
                let left_idx = s.bytes().nth(left).unwrap() as usize;
                if running[left_idx] > counter[left_idx] {
                    running[left_idx] -= 1;
                    left += 1;
                    left_changed = true;
                } else {
                    break;
                }
            }

            // println!("{}: {:?}", i, deque);

            // case 1: if window is found, and something is popped the window may be smaller.
            // case 0: if window is None, check the running counter against the pattern counter,
            // the window may be available

            let may_update = (window.is_some() && left_changed)
                || (window.is_none()
                    && running[ch_idx] >= counter[ch_idx]
                    && running.iter().zip(counter.iter()).all(|(a, b)| a >= b));

            if may_update {
                let start = left;
                let stop = pos + 1;
                let len = stop - start;
                // println!("{}: {} {}", i, start, stop);

                match &window {
                    Some(old_window) => {
                        if old_window.len() > len {
                            window = Some(&s[start..stop]);
                        }
                    }
                    None => {
                        window = Some(&s[start..stop]);
                    }
                }
            }
        }

        window.unwrap_or("").to_owned()
    }

    pub fn min_window2(s: String, t: String) -> String {
        use std::collections::VecDeque;

        const COUNTER_SIZE: usize = 128;

        let counter = t
            .bytes() //
            .into_iter()
            .fold([0; COUNTER_SIZE], |mut counter, ch| {
                counter[ch as usize] += 1;
                counter
            });

        let mut running = [0; COUNTER_SIZE];
        let mut deque = VecDeque::<usize>::new();
        let mut window: Option<&str> = Option::None;

        for (i, ch) in s.bytes().into_iter().enumerate() {
            let ch_idx = ch as usize;
            running[ch_idx] += 1;
            deque.push_back(i);

            let mut popped = false;

            // try to shrink deque
            while let Some(&pos) = deque.front() {
                let front_idx = s.bytes().nth(pos).unwrap() as usize;
                if running[front_idx] > counter[front_idx] {
                    deque.pop_front();
                    running[front_idx] -= 1;
                    popped = true;
                } else {
                    break;
                }
            }
            // println!("{}: {:?}", i, deque);

            // case 1: if window is found, and something is popped the window may be smaller.
            // case 0: if window is None, check the running counter against the pattern counter,
            // the window may be available

            let may_update = (window.is_some() && popped)
                || (window.is_none()
                    && running[ch_idx] >= counter[ch_idx]
                    && running.iter().zip(counter.iter()).all(|(a, b)| a >= b));

            if may_update {
                let start = *deque.front().unwrap_or(&0);
                let stop = i + 1;
                let len = stop - start;
                // println!("{}: {} {}", i, start, stop);

                match &window {
                    Some(old_window) => {
                        if old_window.len() > len {
                            window = Some(&s[start..stop]);
                        }
                    }
                    None => {
                        window = Some(&s[start..stop]);
                    }
                }
            }
        }

        window.unwrap_or("").to_owned()
    }
}

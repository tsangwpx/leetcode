// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 84
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::with_capacity(heights.len()); // index stack whose values are increasing
        let mut maximum = 0;

        for new_idx in 0..heights.len() {
            let new_height = heights[new_idx];

            loop {
                let need_pop = match stack.last() {
                    None => false,
                    Some(&top_idx) => heights[top_idx] > new_height,
                };

                if !need_pop {
                    break;
                }

                let top_idx = stack.pop().unwrap();
                let width = match stack.last() {
                    None => new_idx,
                    Some(left_idx) => new_idx - left_idx - 1,
                };
                maximum = maximum.max(width as i32 * heights[top_idx]);
            }

            stack.push(new_idx);
        }

        while let Some(top_idx) = stack.pop() {
            let width = match stack.last() {
                None => heights.len(),
                Some(left_idx) => heights.len() - left_idx - 1,
            };
            maximum = maximum.max(width as i32 * heights[top_idx]);
        }

        maximum
    }

    pub fn largest_rectangle_area2(heights: Vec<i32>) -> i32 {
        // #[derive(Debug)]
        struct Bound {
            start: usize,
            stop: usize,
        }

        impl std::fmt::Debug for Bound {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("Bound({}, {})", self.start, self.stop))
            }
        }

        let mut bounds: Vec<Bound> = (0..heights.len())
            .into_iter()
            .map(|idx| Bound {
                start: idx,
                stop: idx + 1,
            })
            .collect();

        let mut stack: Vec<usize> = vec![]; // index stack whose values are increasing

        fn loop_once<F>(
            heights: &Vec<i32>,
            bounds: &mut Vec<Bound>,
            stack: &mut Vec<usize>,
            new_idx: usize,
            update: F,
        ) where
            F: Fn(&mut Bound, usize),
        {
            let new_height = heights[new_idx];
            let need_popping = match stack.last() {
                None => false,
                Some(&last_idx) => heights[last_idx] > new_height,
            };

            if need_popping {
                let last_idx = *stack.last().unwrap();

                while let Some(&top_idx) = stack.last() {
                    let top_height = heights[top_idx];
                    if top_height <= new_height {
                        break;
                    }

                    stack.pop();
                    update(&mut bounds[top_idx], last_idx);
                }
            }
            stack.push(new_idx);
        }

        for new_idx in 0..heights.len() {
            loop_once(
                &heights,
                &mut bounds,
                &mut stack,
                new_idx,
                |bound, last_idx| bound.stop = last_idx + 1,
            );
        }

        while let Some(top_idx) = stack.pop() {
            bounds[top_idx].stop = heights.len();
        }

        for new_idx in (0..heights.len()).rev() {
            loop_once(
                &heights,
                &mut bounds,
                &mut stack,
                new_idx,
                |bound, last_idx| bound.start = last_idx,
            );
        }

        while let Some(top_idx) = stack.pop() {
            bounds[top_idx].start = 0;
        }

        // println!("{:?}", bounds);

        let mut maximum = 0;

        for (&height, bound) in heights.iter().zip(bounds.iter()) {
            maximum = maximum.max(height * (bound.stop - bound.start) as i32);
        }

        maximum
    }
}

// Problem 2071
impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        use std::collections::VecDeque;

        if tasks.is_empty() || workers.is_empty() {
            return 0;
        }

        tasks.sort_unstable();
        workers.sort_unstable();

        // println!("{:?}", tasks);
        // println!("{:?}", workers);

        let tasks = tasks;
        let workers = workers;

        let mut left = 0;
        let mut right = tasks.len().min(workers.len());

        #[inline]
        fn check(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32) -> bool {
            assert!(tasks.len() == workers.len());

            // println!("{} {:?} {:?}", tasks.len(), tasks, workers);

            if tasks.is_empty() {
                return true;
            }

            let mut deque = VecDeque::new();
            let mut widx = workers.len().wrapping_sub(1);

            for work in tasks.iter().rev().copied() {
                // fill the deque with workers (probably with pill) that can finish the task
                while widx < workers.len() && workers[widx] + strength >= work {
                    deque.push_front(workers[widx]);
                    widx = widx.wrapping_sub(1);
                }

                if deque.is_empty() {
                    // no worker can finish the work
                    return false;
                }

                if deque.back().copied().unwrap() >= work {
                    // remove the largest worker without pill
                    deque.pop_back();
                } else if pills >= 1 {
                    // remove the smallest worker with pill
                    pills -= 1;
                    deque.pop_front();
                } else {
                    // no pill available
                    return false;
                }
            }

            true
        }

        while left < right {
            let count = (right - left + 1) / 2 + left;
            let weak_tasks = &tasks[0..count];
            let strong_workers = &workers[workers.len() - count..];

            if check(weak_tasks, strong_workers, pills, strength) {
                left = count;
            } else {
                right = count - 1;
            }
        }

        left as _
    }
}

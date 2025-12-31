// Problem 1381
#[derive(Debug, Clone)]
struct CustomStack {
    capacity: usize,
    stack: Vec<i32>,
    changes: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 *
 * In this implementation, the basic stack is provided by Vec.
 * The increment functionality is calculated with `changes` array,
 * which store the delta of the corresponding stack element to the correct value.
 * The delta value also propagate to items below when the stack item is popped.
 */
impl CustomStack {
    fn new(maxSize: i32) -> Self {
        let capacity = maxSize as usize;
        Self {
            capacity: capacity,
            stack: Vec::with_capacity(capacity),
            changes: vec![0; capacity],
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.capacity {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if let Some(x) = self.stack.pop() {
            let idx = self.stack.len();
            let delta = self.changes[idx];

            // zero out the delta
            self.changes[idx] = 0;

            if idx >= 1 {
                // Propagate to items below
                self.changes[idx - 1] += delta;
            }

            x + delta
        } else {
            -1
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = k as usize;
        let k = k.min(self.stack.len());
        if k >= 1 {
            self.changes[k - 1] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
fn __() {}

// Problem 155
#[derive(Default)]
struct MinStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        let minimum = match self.stack.last() {
            Some(&(_, s)) => s.min(val),
            None => val,
        };
        self.stack.push((val, minimum));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        match self.stack.last() {
            Some(&(s, _)) => s,
            None => panic!("Invalid call"),
        }
    }

    fn get_min(&self) -> i32 {
        match self.stack.last() {
            Some(&(_, s)) => s,
            None => panic!("Invalid call"),
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn dummy() {}

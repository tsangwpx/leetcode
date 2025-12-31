// Problem 641
#[derive(Debug, Clone)]
struct MyCircularDeque {
    data: Vec<i32>,
    offset: usize,
    size: usize,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize],
            offset: 0,
            size: 0,
            capacity: k as usize,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.size >= self.capacity {
            return false;
        }

        if self.offset == 0 {
            self.offset = self.capacity - 1;
        } else {
            self.offset -= 1;
        }
        self.data[self.offset] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.size >= self.capacity {
            return false;
        }

        let mut idx = self.offset + self.size;
        if idx >= self.capacity {
            idx -= self.capacity;
        }

        self.data[idx] = value;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }

        self.offset += 1;
        if self.offset >= self.capacity {
            self.offset -= self.capacity;
        }

        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }

        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.size == 0 {
            -1
        } else {
            self.data[self.offset]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.size == 0 {
            -1
        } else {
            let mut idx = self.offset + self.size - 1;
            if idx >= self.capacity {
                idx -= self.capacity;
            }
            self.data[idx]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
fn _unused() {}

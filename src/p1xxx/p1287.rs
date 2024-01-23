// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 1287
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let limit = arr.len() as u32 / 4 + 1;
        let mut table = HashMap::<i32, u32>::with_capacity(10);
        // heap of (size, left, right). Both interval ends are inclusive. primarily sorted by size.
        let mut heap = BinaryHeap::<(usize, usize, usize)>::new();

        heap.push((arr.len(), 0, arr.len() - 1));

        while let Some((_, left, right)) = heap.pop() {
            let left_value = arr[left];
            let right_value = arr[right];

            if left_value == right_value {
                let increment = (right - left + 1) as u32;
                let count = *table
                    .entry(left_value)
                    .and_modify(|s| *s += increment)
                    .or_insert(increment);

                if count >= limit {
                    return left_value;
                }
            } else {
                // Now, left != right
                // We split [left, right] into two subintervals: [left, mid] and (mid, right]
                // Note that the inequality holds: left <= mid < right
                let mid = (left + right) / 2;
                heap.push((right - mid, mid + 1, right));
                heap.push((mid - left + 1, left, mid));
            }
        }

        panic!("No items found")
    }
}

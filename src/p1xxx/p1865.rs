// Problem 1865
use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    counter2: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let counter2 = nums2.iter().copied().fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_default() += 1;
            map
        });

        Self {
            nums1,
            nums2,
            counter2,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let old_value = self.nums2[index];
        let new_value = old_value + val;
        self.nums2[index] = new_value;

        {
            let old_count = self.counter2.entry(old_value).or_default();
            *old_count -= 1;
            if *old_count == 0 {
                self.counter2.remove(&old_value);
            }
        }

        *self.counter2.entry(new_value).or_default() += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut count = 0;

        for item in self.nums1.iter().copied() {
            let other = tot - item;
            count += self.counter2.get(&other).copied().unwrap_or(0);
        }

        count
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
fn _f() {}

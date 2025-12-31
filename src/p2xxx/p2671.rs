// Problem 2671
use std::collections::HashMap;

#[derive(Default, Debug)]
struct FrequencyTracker {
    num_counter: HashMap<i32, i32>,
    freq_counter: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
    fn new() -> Self {
        Default::default()
    }

    fn _update_frequency(&mut self, freq: i32, increase: bool) {
        if let Some(count) = self.freq_counter.get_mut(&freq) {
            *count += if increase { 1 } else { -1 };
            if *count <= 0 {
                self.freq_counter.remove(&freq);
            }
        } else {
            if increase {
                *self.freq_counter.entry(freq).or_default() += 1;
            }
        }
    }

    fn add(&mut self, number: i32) {
        let count = self.num_counter.entry(number).or_default();
        let old_count = *count;

        *count += 1;

        let new_count = *count;

        self._update_frequency(old_count, false);
        self._update_frequency(new_count, true);

        // println!("{:?}", self.freq_counter);
    }

    fn delete_one(&mut self, number: i32) {
        if let Some(count) = self.num_counter.get_mut(&number) {
            let old_count = *count;
            *count -= 1;
            let new_count = *count;

            self._update_frequency(old_count, false);
            self._update_frequency(new_count, true);

            if new_count == 0 {
                self.num_counter.remove(&number);
            }
        }

        // println!("{:?}", self.freq_counter);
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.freq_counter
            .get(&frequency)
            .map(|&s| s >= 1)
            .unwrap_or(false)
    }
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */
fn _f() {}

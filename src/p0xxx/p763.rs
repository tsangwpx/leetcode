fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 763
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        assert!(s.len() >= 1);

        let mut last_occurrence = [0usize; 128];

        for (i, ch) in s.bytes().into_iter().enumerate() {
            last_occurrence[ch as usize] = i;
        }

        let mut partitions = vec![];
        let mut current_start = 0;
        let mut current_stop = 1;

        for (i, ch) in s.bytes().into_iter().enumerate() {
            if i >= current_stop {
                partitions.push((current_stop - current_start) as i32);
                current_start = i;
            }
            current_stop = current_stop.max(last_occurrence[ch as usize] + 1);
        }

        if current_stop > current_start {
            partitions.push((current_stop - current_start) as i32);
        }

        partitions
    }
}

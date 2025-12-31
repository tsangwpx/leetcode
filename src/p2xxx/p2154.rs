// Problem 2154
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        use std::collections::HashMap;

        let mut table = HashMap::new();

        let mut target = original;

        loop {
            table.insert(target, false);

            if let Some(new_target) = target.checked_mul(2) {
                target = new_target;
            } else {
                break;
            }
        }

        for item in nums.iter().copied() {
            if let Some(found) = table.get_mut(&item) {
                *found = true;
            }
        }

        table
            .iter()
            .filter_map(|(&k, &v)| if !v { Some(k) } else { None })
            .min()
            .expect("overflow")
    }
}

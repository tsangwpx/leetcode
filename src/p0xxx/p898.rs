// Problem 898
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut seen = HashSet::new();
        let mut state = Vec::with_capacity(32);

        for item in arr.iter().copied() {
            for num in state.iter_mut() {
                *num |= item;
            }

            state.push(item);
            state.dedup();

            seen.extend(state.iter().copied());
        }

        seen.len() as _
    }
}

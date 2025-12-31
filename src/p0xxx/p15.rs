// Problem 15
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Returns a list of unique triples (nums[i], nums[j], nums[k])
        // where nums[i] <= nums[j] <= nums[k] and i != j, j !=k, k != i

        use std::collections::HashMap;

        nums.sort_unstable();

        let mut res = vec![];

        // save the last occurrence of numbers
        let table = nums
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, num)| (num, idx))
            .collect::<HashMap<_, _>>();

        let mut idx1 = 0;
        while idx1 < nums.len() {
            let num1 = nums[idx1];
            if num1 > 0 {
                // nums is sorted, the numbers afterward are all positive
                break;
            }

            let mut idx2 = idx1 + 1;
            while idx2 < nums.len() {
                let num2 = nums[idx2];
                let num3 = -(num1 + num2);

                if let Some(idx3) = table.get(&num3).copied() {
                    if idx3 > idx2 {
                        res.push(vec![num1, num2, num3]);
                    }
                }

                // fast forward to next different numbers
                idx2 = table.get(&num2).unwrap() + 1;
            }

            // fast forward to next difference numbers
            idx1 = table.get(&num1).unwrap() + 1;
        }

        res
    }
}

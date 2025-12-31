// Problem 78
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn is_present(mask: u32, shift: u32) -> bool {
            (mask >> shift) & 1 == 1
        }

        let total = 2u32.pow(nums.len() as u32);
        // let mut res = Vec::with_capacity(total as usize);
        let mut res = vec![];
        let mut work = vec![];

        for mask in 0..total {
            work.clear();

            for shift in 0..nums.len() {
                if is_present(mask, shift as u32) {
                    work.push(nums[shift]);
                }
            }

            res.push(work.clone());
        }

        res
    }
}

// Problem 1437
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut prev = None;

        for (idx, item) in nums.iter().copied().enumerate() {
            if item == 1 {
                if let Some(prev) = prev {
                    let step = idx - prev - 1;

                    if step < k {
                        return false;
                    }
                }

                prev = Some(idx);
            }
        }

        true
    }
}

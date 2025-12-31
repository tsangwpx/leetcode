impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::{HashMap, HashSet};

        let total_sum: i32 = nums.iter().sum();
        if total_sum % k != 0 {
            return false;
        }
        let target = total_sum / k;
        let mut failed = HashSet::<u32>::new();

        fn recurse(
            nums: &Vec<i32>,
            target: i32,
            failed: &mut HashSet<u32>,
            visited: u32,
            mut incomplete: u32,
            mut start: usize,
            mut bin_sum: i32,
        ) -> bool {
            if bin_sum == target {
                // this bin reach target, start another bin from the beginning
                incomplete -= 1;
                start = 0;
                bin_sum = 0;
            }

            if incomplete == 0 {
                // all bins are complete
                return true;
            }

            for i in start..nums.len() {
                let flag = 1 << i;
                if visited & flag == flag {
                    continue;
                }

                let bin_sum2 = bin_sum + nums[i];
                if bin_sum2 > target {
                    // skip too large item
                    continue;
                }

                let visited2 = visited | flag;
                if failed.contains(&visited2) {
                    // skip failed configuration
                    continue;
                }

                if recurse(nums, target, failed, visited2, incomplete, i + 1, bin_sum2) {
                    // found so return directly
                    return true;
                }
            }

            // record the failed configuration
            failed.insert(visited);

            false
        }

        recurse(&nums, target, &mut failed, 0, k as u32, 0, 0)
    }
}

fn main() {
    println!("Hello World");
}

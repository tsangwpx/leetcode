// Problem 45
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);

        let dest = nums.len() - 1;

        if dest == 0 {
            return 0;
        }

        let mut count = 0;
        let mut jump_land = 0; // farthest index where the current jump land
        let mut jump_max = 0; // farthest index where another jump take us to.

        for i in 0..nums.len() {
            let reachable = nums[i] as usize + i;
            jump_max = jump_max.max(reachable);

            if i == jump_land {
                // We have to take an another jump here or somewhere before
                // such that we could land in jump_max

                if jump_max <= i {
                    // No jump allow us to go beyond this index
                    break;
                }

                count += 1;
                jump_land = jump_max;

                if jump_land >= dest {
                    break;
                }
            }
        }

        if jump_land >= dest {
            count
        } else {
            unreachable!("Impossible!")
        }
    }

    pub fn jump3(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);

        let mut indices = (0..nums.len()).collect::<Vec<usize>>();
        indices.sort_by_key(|&s| s + nums[s] as usize);
        let mut index_stop = indices.len();

        let mut count = 0;
        let mut pos = nums.len() - 1;

        while pos > 0 {
            let mut smallest_nonzero = pos;

            for ii in (0..index_stop).rev() {
                let idx = indices[ii];
                if nums[idx] == 0 {
                    continue;
                }
                if idx + nums[idx] as usize >= pos {
                    smallest_nonzero = smallest_nonzero.min(idx);
                } else {
                    index_stop = ii + 1;
                    break;
                }
            }

            if smallest_nonzero == pos {
                unreachable!("No improvement!");
            }

            count += 1;
            pos = smallest_nonzero;
        }

        count
    }

    pub fn jump2(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        let mut dp = vec![Option::None; nums.len()];
        dp[0] = Some(0);

        for idx in 0..nums.len() - 1 {
            if let Some(count) = dp[idx] {
                let step_max = nums[idx] as usize;

                for step in 1..=step_max {
                    let dest = idx + step;

                    // println!("{}: {} + {}", dest, idx, step);

                    if dest >= nums.len() {
                        break;
                    }

                    dp[dest] = match dp[dest] {
                        Some(old_count) => Some(old_count.min(count + 1)),
                        None => Some(count + 1),
                    }
                }
            }
        }

        // println!("{:?}", dp);

        // test case is able to reach the end
        match dp[nums.len() - 1] {
            Some(jumps) => jumps,
            None => unreachable!(),
        }
    }
}

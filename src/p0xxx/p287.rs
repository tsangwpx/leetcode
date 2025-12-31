// Problem 287
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Tortoise and Hare Algorithm
        // Hare moves twice as fast as tortoise
        // nums is a subset of [1, n] and nums length is n + 1, there must be duplicated numbers.
        // the 0-th index is one of the special positions where no other node pointing to them.
        // They will meet somewhere in a loop
        //
        // In general, the path they travel look like:
        // S ---> X ---> M --> X
        // S is the starting point (the index 0)
        // X is the duplicate number or the index pointed by two or more nodes
        // M is the index where they meet.
        //
        // L = the length from S to X
        // C = the length of the loop
        // m = the length from X to M
        //
        // Hare travel distance is: H = L + h * C + m, for some integers h.
        // Tortoise distance is: T = L + t * C + m, for some integers t.
        // Since hare moves fast, we have H = 2 * T
        // Simplify the equation, we have (h - t) * C = L + m
        //
        // This means the L + m is a multiple of loop length!
        // or, if one of them moves more L steps, it would reach X, which is the duplicate.
        //
        // So what is L? We dont need to know.
        // Ask the other back to the beginning, the zero position, and steps L times
        // They will meet again where is the duplicate.

        let mut tort = 0;
        let mut hare = 0;

        loop {
            tort = nums[tort as usize];
            hare = nums[nums[hare as usize] as usize];

            if tort == hare {
                break;
            }
        }

        hare = 0;
        loop {
            tort = nums[tort as usize];
            hare = nums[hare as usize];

            if tort == hare {
                break;
            }
        }

        tort
    }

    pub fn find_duplicate2(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; nums.len()];
        for number in nums {
            if seen[number as usize] {
                return number;
            }

            seen[number as usize] = true;
        }

        unreachable!()
    }

    pub fn find_duplicate3(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut seen = HashSet::with_capacity(nums.len());

        for number in nums {
            if !seen.insert(number) {
                return number;
            }
        }

        unreachable!()
    }
}

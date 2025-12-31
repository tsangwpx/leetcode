// Problem 1404
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut steps = 0;
        let mut idx = s.len() - 1;
        let mut idx_is_one = false;

        // since the leading digit must be one
        // we terminate the loop when idx = 0
        while idx < s.len() && idx > 0 {
            let ch = s.bytes().nth(idx).unwrap();

            if idx_is_one || ch == b'1' {
                // Count how many ones
                // the first one may be real or virtual.
                let mut ones = 1;
                idx = idx.wrapping_sub(1);

                while idx < s.len() && s.bytes().nth(idx).unwrap() == b'1' {
                    ones += 1;
                    idx = idx.wrapping_sub(1);
                }

                // the number of steps to consume the tail is
                // one more than the number of ones
                steps += ones + 1;

                // Put the carry in the next zero position, and mark it as one.
                idx_is_one = true;
            } else {
                steps += 1;
                idx = idx.wrapping_sub(1);
            }
        }

        steps
    }
}

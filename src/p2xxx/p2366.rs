// Problem 2366
impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut count = 0;

        let mut bound = nums.last().copied().unwrap();

        for idx in (0..nums.len().saturating_sub(1)).rev() {
            let number = nums[idx];

            if number <= bound {
                bound = number;
                continue;
            }

            let (q, r) = (number / bound, number % bound);
            // println!("{} {} {} {}", q, r, number, bound);

            // how many steps are needed?
            if r == 0 {
                // If remainder is zero, the number is a multiple of bound
                // The bound is unchanged and (q - 1) steps are needed
                count += (q - 1) as i64;
            } else {
                // Otherwise, q steps is needed
                let steps = q;
                count += steps as i64;

                // and the new bound is the quotient of number / (steps + 1)
                bound = number / (steps + 1)
            }
        }

        count
    }
}

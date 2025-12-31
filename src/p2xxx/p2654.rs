// Problem 2654
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        fn gcd(mut u: i32, mut v: i32) -> i32 {
            // https://en.wikipedia.org/wiki/Binary_GCD_algorithm
            if u == 0 {
                return v;
            } else if v == 0 {
                return u;
            }

            let i = u.trailing_zeros();
            let j = v.trailing_zeros();
            let k = i.min(j);

            u >>= i;
            v >>= j;

            loop {
                if u > v {
                    std::mem::swap(&mut u, &mut v);
                }

                v -= u;

                if v == 0 {
                    return u << k;
                }

                v >>= v.trailing_zeros();
            }
        }

        // If there are ones, it spread to convert non-ones to ones.
        let one_count = nums.iter().filter(|&&n| n == 1).count();
        if one_count >= 1 {
            return (nums.len() - one_count) as i32;
        }

        let mut left = 0;
        let mut min_operations = usize::MAX;

        while left < nums.len() {
            // [left, right) is the sliding window of GCD.
            // Scan to determine the right endpoint of the minimum window
            let mut right = left + 1;
            let mut g = nums[left];

            while g != 1 && right < nums.len() {
                g = gcd(g, nums[right]);
                right += 1;
            }

            if g != 1 {
                // right == nums.len()
                break;
            }

            // backtrack the GCD to the left side
            left = right - 1;
            g = nums[left];

            loop {
                if g == 1 || left == 0 {
                    break;
                }

                left -= 1;

                g = gcd(g, nums[left]);
            }

            assert!(g == 1);
            // now GCD = 1 in [left, right)

            // println!("{} {} {}", left, right, g);

            let op_count = right - left - 1;
            min_operations = min_operations.min(op_count);

            // bump the left so that GCD is no longer 1
            left += 1;
        }

        if min_operations == usize::MAX {
            -1
        } else {
            (min_operations + nums.len() - 1) as i32
        }
    }
}

// Problem 869
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        use std::collections::HashSet;

        fn get_signature(mut n: i32) -> [u8; 10] {
            let mut res = [0u8; 10];

            while n >= 1 {
                res[n as usize % 10] += 1;
                n /= 10;
            }

            res
        }

        let target = get_signature(n);

        for shift in 0..32 {
            let n = 1 << shift;
            if get_signature(n) == target {
                return true;
            }
        }

        false
    }
}

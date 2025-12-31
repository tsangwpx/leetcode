// Problem 416
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().copied().sum::<i32>();

        if sum % 2 == 1 {
            return false;
        }

        let target = sum as usize / 2;

        let size = {
            let count = target + 1;
            let (mut q, r) = (count / 32, count % 32);

            if r != 0 {
                q += 1;
            }

            q
        };
        let mut bits = vec![0u32; size];
        bits[0] |= 1;

        for number in nums.iter().copied() {
            let number = number as usize;

            if number > target {
                continue;
            }

            let (q, r) = (number / 32, number % 32);

            for idx in (0..bits.len()).rev() {
                let mut mask = bits[idx];

                if r == 0 {
                    if idx >= q {
                        mask |= bits[idx - q];
                    }
                } else {
                    if idx >= q {
                        mask |= bits[idx - q] << r;
                    }
                    if idx >= q + 1 {
                        mask |= bits[idx - q - 1] >> (32 - r);
                    }
                }

                bits[idx] = mask;
            }
        }

        let (q, r) = (target / 32, target % 32);

        (bits[q] >> r) & 1 == 1
    }
}

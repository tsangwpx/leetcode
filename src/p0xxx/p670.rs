// Problem 670
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = num.to_string().chars().collect::<Vec<_>>();

        // the rightmost index whose value is the largest
        let mut right = digits.len() - 1;

        // the possible swap
        let mut pair = None;

        for idx in (0..digits.len()).rev() {
            if (digits[idx] as u32) < (digits[right] as u32) {
                // found possible swap
                pair = Some((idx, right));
            }

            if (digits[idx] as u32) > (digits[right] as u32) {
                // found larger digit
                right = idx;
            }
        }

        if let Some((left, right)) = pair {
            digits.swap(left, right);
        }

        digits.into_iter().fold(0, |mut num, ch| {
            num *= 10;
            num += ((ch as u32) - ('0' as u32)) as i32;
            num
        })
    }
}

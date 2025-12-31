// Problem 2048
impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        'outer: for i in n + 1.. {
            let mut counter = [0; 10];
            let mut rem = i as u32;

            while rem >= 1 {
                let (q, r) = (rem / 10, rem % 10);
                let idx = r as usize;

                if counter[idx] >= idx || idx >= 8 {
                    continue 'outer;
                }

                counter[idx] += 1;
                rem = q;
            }

            for digit in 0..8 {
                let count = counter[digit];
                if count > 0 && count != digit {
                    continue 'outer;
                }
            }

            return i;
        }

        unreachable!()
    }
}

// Problem 2843
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut count = 0;
        let mut buf = vec![];
        for num in low..=high {
            buf.clear();

            let mut rem = num;

            while rem != 0 {
                let (q, r) = (rem / 10, rem % 10);
                buf.push(r as u8);
                rem = q;
            }

            if buf.len() % 2 == 1 {
                continue;
            }
            let h = buf.len() / 2;

            let left_sum = buf[..h]
                .iter()
                .copied()
                .fold(0i32, |acc, ch| acc + ch as i32);
            let right_sum = buf[h..]
                .iter()
                .copied()
                .fold(0i32, |acc, ch| acc + ch as i32);

            if left_sum == right_sum {
                count += 1;
            }
        }

        count
    }
}

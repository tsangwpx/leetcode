// Problem 1432
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let (buf, buf_len) = {
            let mut buf = [0u8; 10];
            let mut len = 0;
            let mut rem = num;

            while rem != 0 {
                let (q, r) = (rem / 10, rem % 10);
                buf[len] = r as u8;
                len += 1;
                rem = q;
            }

            buf[0..len].reverse();
            (buf, len)
        };

        let mut min = num;
        let mut max = num;

        for src in 0..10 {
            for dst in 0..10 {
                if src == dst || (buf[0] == src && dst == 0) {
                    continue;
                }

                let new_num = buf[0..buf_len].iter().fold(0i32, |acc, &digit| {
                    let digit = if digit == src { dst } else { digit };
                    acc * 10 + digit as i32
                });

                min = min.min(new_num);
                max = max.max(new_num);
            }
        }

        max - min
    }
}

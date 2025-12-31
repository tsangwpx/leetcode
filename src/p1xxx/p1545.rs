// Problem 1545
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        // Since n <= 20
        // the length of S_20 is 41 bits
        // we may fit the whole number in u32
        // but flipping u32 left and right is a bit complicated (need google the formula)
        // and need some correct shifting
        // so vec is used here

        let mut s0 = vec!['0'];
        let mut s1 = vec![];

        for _ in 1..=n {
            s1.clear();
            s1.reserve(s0.len() * 2 + 1);
            s1.extend_from_slice(&s0[..]);
            s1.push('1');
            s1.extend(
                s0.iter()
                    .rev()
                    .copied()
                    .map(|s| if s == '0' { '1' } else { '0' }),
            );

            std::mem::swap(&mut s0, &mut s1);
        }

        s0[k as usize - 1]
    }
}

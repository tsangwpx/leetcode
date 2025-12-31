// Problem 3234
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        // https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/solutions/5548547/well-explained-very-easy-java-by-px_07-q1sh/?envType=daily-question&envId=2025-11-15

        let n = s.len();
        let pfxsums = s.bytes().fold(vec![0], |mut acc, item| {
            let last = acc.last().copied().unwrap();
            acc.push(last + (item - b'0') as i32);
            acc
        });
        let mut count = 0;

        for i in 0..n {
            let mut j = i;

            while j < n {
                let one = pfxsums[j + 1] - pfxsums[i];
                let zero = (j + 1 - i) as i32 - one;

                let delta = zero * zero - one;

                match delta.cmp(&0) {
                    std::cmp::Ordering::Greater => {
                        j += (zero * zero - one - 1) as usize;
                    }
                    std::cmp::Ordering::Equal => {
                        count += 1;
                    }
                    std::cmp::Ordering::Less => {
                        count += 1;

                        let step = one.isqrt() - zero;
                        let j2 = j + step as usize;

                        if j2 >= n {
                            count += (n - j - 1) as i32;
                        } else {
                            count += step;
                        }

                        j = j2;
                    }
                }

                j += 1;
            }
        }

        count
    }
}

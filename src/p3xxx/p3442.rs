// Problem 3442
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut counter = [0; 26];
        s.bytes().for_each(|s| counter[(s - b'a') as usize] += 1);

        let (a1, a2) = counter
            .iter()
            .fold((0, i32::MAX), |(mut a1, mut a2), &count| {
                if count != 0 {
                    if count % 2 == 0 {
                        a2 = a2.min(count);
                    } else {
                        a1 = a1.max(count);
                    }
                }

                (a1, a2)
            });

        a1 - a2
    }
}

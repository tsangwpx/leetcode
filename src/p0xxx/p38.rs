// Problem 38
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s0 = "1".to_owned();
        let mut s1 = String::new();

        for _ in 1..n {
            s1.clear();

            let mut it = s0.chars().peekable();

            while let Some(ch) = it.next() {
                let mut count = 1;
                while let Some(_) = it.next_if_eq(&ch) {
                    count += 1;
                    if count == 9 {
                        break;
                    }
                }

                s1.push(char::from_digit(count, 10).unwrap());
                s1.push(ch);
            }

            std::mem::swap(&mut s0, &mut s1);
        }

        s0
    }
}

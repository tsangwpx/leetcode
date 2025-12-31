// Problem 1717
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        // make sure x >= y. otherwise, swap a and b so are x and y
        let (a, b, x, y) = if x >= y {
            ('a', 'b', x, y)
        } else {
            ('b', 'a', y, x)
        };

        let mut score = 0;
        let mut stack = vec![];
        for ch in s.chars() {
            if ch == b && stack.last().copied() == Some(a) {
                stack.pop();
                score += x;
            } else {
                stack.push(ch);
            }
        }

        let mut stack2 = vec![];
        for ch in stack.into_iter() {
            if ch == a && stack2.last().copied() == Some(b) {
                stack2.pop();
                score += y;
            } else {
                stack2.push(ch);
            }
        }

        score
    }
}

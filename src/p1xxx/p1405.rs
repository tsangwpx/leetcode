// Problem 1405
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        use std::cmp::Reverse;

        let mut queue = [(Reverse(a), b'a'), (Reverse(b), b'b'), (Reverse(c), b'c')];

        let mut res = String::new();

        'outer: loop {
            // constant time sorting
            queue.sort_unstable();

            // iterate all 3 items
            for (idx, (Reverse(count), ch)) in queue.iter().copied().enumerate() {
                if count == 0 {
                    continue;
                }

                if res.bytes().nth_back(1) == Some(ch) && res.bytes().nth_back(0) == Some(ch) {
                    continue;
                }

                res.push(ch as char);
                queue[idx] = (Reverse(count - 1), ch);

                continue 'outer;
            }

            break 'outer;
        }

        res
    }
}

// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 56
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        intervals.sort_by_key(|x| match x[..] {
            [a, b] => (b, a),
            _ => unreachable!(),
        });

        while let Some(mut pair) = intervals.pop() {
            assert!(pair.len() == 2);
            let mut leftmost = pair[0];

            while let Some((left, right)) = intervals.last().map(|s| (s[0], s[1])) {
                if right < leftmost {
                    break;
                }

                leftmost = left.min(leftmost);
                intervals.pop();
            }

            pair[0] = leftmost;
            ans.push(pair);
        }

        ans
    }
}

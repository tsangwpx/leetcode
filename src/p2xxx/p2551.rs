// Problem 2551
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        if weights.len() <= 2 {
            return 0;
        }

        let mut neighbours = vec![];

        for x in weights.windows(2) {
            neighbours.push(x.iter().sum::<i32>());
        }

        neighbours.sort_unstable();

        let mut res = 0i64;

        for (front, back) in neighbours
            .iter()
            .copied()
            .take(k as usize - 1)
            .zip(neighbours.iter().copied().rev().take(k as usize - 1))
        {
            res += i64::from(back);
            res -= i64::from(front);
        }

        res
    }
}

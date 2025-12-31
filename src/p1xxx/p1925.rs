// Problem 1925
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        let mut squares = vec![];

        for c in 1..=n {
            let c2 = c * c;

            let mut left = 0;
            let mut right = squares.len() - 1;

            while left < squares.len() && right < squares.len() && left < right {
                let delta: i32 = squares[left] + squares[right] - c2;

                if delta == 0 {
                    count += 2;
                }

                if delta <= 0 {
                    left += 1;
                }

                if delta >= 0 {
                    right -= 1;
                }
            }

            squares.push(c2);
        }

        count
    }
}

// Problem 875

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        assert!(piles.len() >= 1);
        let mut lo = 1;
        let mut hi = piles.iter().copied().max().unwrap();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            let time_required = piles
                .iter()
                .map(|&bananas| (bananas + mid - 1) / mid)
                .sum::<i32>();

            if time_required > h {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo
    }
}

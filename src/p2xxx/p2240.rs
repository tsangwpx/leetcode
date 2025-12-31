// Problem 2240
impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut count = 0;

        let mut pens = 0;

        loop {
            let rem = total - pens * cost1;
            if rem < 0 {
                break;
            }
            let pencils = rem / cost2;

            count += i64::from(pencils) + 1;
            pens += 1;
        }

        count
    }
}

// Problem 1518
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut done = 0;
        let mut empty_bottles = 0;

        while num_bottles > 0 {
            done += num_bottles;

            empty_bottles += num_bottles;

            let (q, r) = (empty_bottles / num_exchange, empty_bottles % num_exchange);
            empty_bottles = r;
            num_bottles = q;
        }

        done
    }
}

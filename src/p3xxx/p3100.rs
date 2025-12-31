// Problem 3100
impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut done = 0;
        let mut empty_bottles = 0;

        while num_bottles > 0 {
            done += num_bottles;
            empty_bottles += num_bottles;
            num_bottles = 0;

            while empty_bottles >= num_exchange {
                num_bottles += 1;
                empty_bottles -= num_exchange;
                num_exchange += 1;
            }
        }

        done
    }
}

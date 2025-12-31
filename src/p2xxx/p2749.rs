// Problem 2749
impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let num1 = i64::from(num1);
        let num2 = i64::from(num2);

        for k in 1..90 {
            let num3 = num1 - num2 * k;

            if num3 >= k && k >= num3.count_ones() as i64 {
                return k as i32;
            }
        }

        -1
    }
}

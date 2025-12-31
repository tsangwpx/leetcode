// Problem 2169
impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        //
        let mut a = num1;
        let mut b = num2;
        let mut count = 0;

        while a != 0 && b != 0 {
            if a > b {
                (a, b) = (b, a);
            }

            let (q, r) = (b / a, b % a);
            count += q;
            b = a;
            a = r;
        }

        count
    }
}

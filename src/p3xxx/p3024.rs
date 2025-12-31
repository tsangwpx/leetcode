// Problem 3024
impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let &[a, b, c] = nums.as_slice() else {
            unreachable!();
        };

        if a == b && b == c {
            "equilateral".to_owned()
        } else if a + b <= c || b + c <= a || c + a <= b {
            "none".to_owned()
        } else if a == b || b == c || c == a {
            "isosceles".to_owned()
        } else {
            "scalene".to_owned()
        }
    }
}

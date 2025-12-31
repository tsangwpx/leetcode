// Problem 3370
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut x = n;
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        x
    }
}

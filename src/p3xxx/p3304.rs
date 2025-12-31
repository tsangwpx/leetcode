// Problem 3304
impl Solution {
    pub fn kth_character(k: i32) -> char {
        (b'a' + (k - 1).count_ones() as u8) as char
    }
}

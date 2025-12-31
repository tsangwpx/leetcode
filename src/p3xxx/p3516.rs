// Problem 3516
impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let dx = (x - z).abs();
        let dy = (y - z).abs();

        match dx.cmp(&dy) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 2,
        }
    }
}

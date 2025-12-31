// Problem 1780
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        // since n <= 10 ** 7
        // and 3 * 10 ** 7 < 2 ** 31
        // we can build powers of 3 in decreasing and store them in i32
        let table = [
            4782969, 1594323, 531441, 177147, 59049, 19683, 6561, 2187, 729, 243, 81, 27, 9, 3, 1,
        ];

        let mut rem = n;
        for number in table.iter().copied() {
            if rem >= number {
                rem -= number;
            }
        }

        rem == 0
    }
}

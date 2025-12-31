// Problem 2145
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        use std::convert::TryFrom;

        let (_, min, max) =
            differences
                .iter()
                .copied()
                .fold((0i64, 0i64, 0i64), |(value, min, max), delta| {
                    let value = value + i64::from(delta);
                    let min = min.min(value);
                    let max = max.max(value);
                    (value, min, max)
                });

        let range = max - min + 1;
        let range2 = i64::from(upper - lower + 1);

        let result = range2 - range + 1;
        if result <= 0 {
            0
        } else {
            i32::try_from(result).unwrap()
        }
    }
}

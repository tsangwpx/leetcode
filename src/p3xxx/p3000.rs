// Problem 3000
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let (_square_sum, area) = dimensions
            .iter()
            .map(|s| {
                let &[a, b] = s.as_slice() else { panic!() };

                (a * a + b * b, a * b)
            })
            .max()
            .unwrap();

        area
    }
}

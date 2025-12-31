// Problem 3195
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut i0, mut i1, mut j0, mut j1) = (usize::MAX, usize::MIN, usize::MAX, usize::MIN);

        for (i, row) in grid.iter().enumerate() {
            for (j, item) in row.iter().copied().enumerate() {
                if item == 1 {
                    i0 = i0.min(i);
                    i1 = i1.max(i);
                    j0 = j0.min(j);
                    j1 = j1.max(j);
                }
            }
        }

        let area = (i1 - i0 + 1) * (j1 - j0 + 1);
        area as _
    }
}

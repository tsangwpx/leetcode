fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 74
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;

        let mut lo = 0;
        let mut hi = matrix.len();

        // This is bisect left, result is smallest index to insert while order is kept.
        while lo < hi {
            let mid = (lo + hi) / 2;

            match matrix[mid].first().unwrap().cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
            }
        }

        // the target row is offset by one.
        // if lo == 0, the target is not found.
        if lo == 0 {
            return false;
        }

        let row = &matrix[lo - 1];
        let mut lo = 0;
        let mut hi = row.len();
        while lo < hi {
            let mid = (lo + hi) / 2;

            match row[mid].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
            }
        }

        false
    }

    pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.binary_search_by_key(&target, |row| *row.first().unwrap()) {
            Ok(_) => true,
            Err(idx) => idx > 0 && matrix[idx - 1].binary_search(&target).is_ok(),
        }
    }
}

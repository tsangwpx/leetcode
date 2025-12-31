// Problem 944
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();

        (0..m)
            .filter(|&j| !(0..n).is_sorted_by_key(|i| strs[i].bytes().nth(j).unwrap()))
            .count() as i32
    }
}

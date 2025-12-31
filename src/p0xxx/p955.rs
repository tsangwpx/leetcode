// Problem 955
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();

        let mut count = 0;
        let mut lt = vec![false; n];
        let mut lt2 = lt.clone();

        'outer: for j in 0..m {
            lt2.clone_from(&lt);

            for i in 0..n.saturating_sub(1) {
                let top = strs[i].bytes().nth(j).unwrap();
                let bot = strs[i + 1].bytes().nth(j).unwrap();

                if top > bot && !lt[i] {
                    // row[i] is greater or equal to row[i + 1]
                    // so adding top to row[i] and bot to row[i + 1] does not work
                    // drop this column
                    count += 1;

                    continue 'outer;
                } else {
                    // if top < bot, then row[i] < row[j] from now on.
                    lt2[i] |= top < bot;
                }
            }

            std::mem::swap(&mut lt, &mut lt2);
        }

        count
    }
}

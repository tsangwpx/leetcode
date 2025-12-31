// Problem 2106
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let fruits = fruits.into_iter().map(|s| (s[0], s[1])).collect::<Vec<_>>();
        let mut pfxsums = vec![0; fruits.len() + 1];
        for i in 0..fruits.len() {
            pfxsums[i + 1] = pfxsums[i] + fruits[i].1;
        }

        // bisect right about start_pos
        // mid is never a pos with fruit
        let mid = fruits.partition_point(|&(pos, _)| pos <= start_pos);

        let mut max_fruits = 0;

        // idx2 is the index after turning pos
        // in the left-then-right phase, mid is the first possible value
        // idx2 is increasing to fruits.len()
        // in the right-then-left phase, 0 is the first possible value
        // idx2 is increasing to mid
        let mut idx2 = mid;

        // in the second phase, we need to reset idx2 to 0 once
        let mut has_idx2_reset = false;

        for idx in 0..fruits.len() {
            let (turning_pos, _) = fruits[idx];
            let d1 = (start_pos - turning_pos).abs();

            if d1 > k {
                // unreachable
                continue;
            }

            // note that mid is never a pos
            let fruit = if turning_pos <= start_pos {
                pfxsums[mid] - pfxsums[idx]
            } else {
                pfxsums[idx + 1] - pfxsums[mid]
            };

            max_fruits = max_fruits.max(fruit);

            if d1 * 2 > k {
                // cant go back to start_pos
                continue;
            }

            if turning_pos <= start_pos {
                // left, then right

                // if idx2 is valid, get fruits, and bump idx2
                while idx2 < fruits.len()
                    && ((start_pos - turning_pos) + (fruits[idx2].0 - turning_pos)) <= k
                {
                    max_fruits = max_fruits.max(fruit + pfxsums[idx2 + 1] - pfxsums[mid]);
                    idx2 += 1;
                }
            } else {
                // right, then left
                if !has_idx2_reset && idx2 >= mid {
                    // if idx2 is on the right side, reset it to 0 once
                    idx2 = 0;
                    has_idx2_reset = true;
                }

                // increase idx2 such that idx2 is valid
                while idx2 < mid && (turning_pos - start_pos) * 2 + (start_pos - fruits[idx2].0) > k
                {
                    idx2 += 1;
                }

                // if idx2 is valid, get the fruits
                if idx2 < mid {
                    max_fruits = max_fruits.max(fruit + pfxsums[mid] - pfxsums[idx2]);
                }
            }
        }

        max_fruits
    }
}

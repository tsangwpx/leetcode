// Problem 1007
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut top_targets = [0; 6];
        let mut bot_targets = [0; 6];

        let n = tops.len().min(bottoms.len());

        for i in 0..n {
            for j in 0..6usize {
                let target = j as i32 + 1;

                if tops[i] != target && bottoms[i] != target {
                    top_targets[j] = -1;
                    bot_targets[j] = -1;
                }

                if top_targets[j] >= 0 && tops[i] != target && bottoms[i] == target {
                    top_targets[j] += 1;
                }

                if bot_targets[j] >= 0 && bottoms[i] != target && tops[i] == target {
                    bot_targets[j] += 1;
                }
            }
            // println!("{} {:?} {:?}", i, top_targets, bot_targets);
        }

        let minimum = top_targets
            .iter()
            .chain(bot_targets.iter())
            .copied()
            .filter(|&s| s >= 0)
            .min();
        minimum.unwrap_or(-1)
    }
}

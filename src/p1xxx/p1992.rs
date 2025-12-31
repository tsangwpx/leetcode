// Problem 1992
impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const FOREST: i32 = 0;
        const FARM: i32 = 1;

        let mut res = vec![];

        for (i, row) in land.iter().enumerate() {
            for (j, item) in row.iter().copied().enumerate() {
                if item == FOREST {
                    continue;
                }

                if (i == 0 || land[i - 1][j] == FOREST) && (j == 0 || row[j - 1] == FOREST) {
                    let left = j;
                    let mut right = j;
                    while right + 1 < row.len() && row[right + 1] == FARM {
                        right += 1;
                    }

                    let top = i;
                    let mut bottom = i;
                    while bottom + 1 < land.len() && land[bottom + 1][j] == FARM {
                        bottom += 1;
                    }

                    res.push(vec![top as i32, left as i32, bottom as i32, right as i32]);
                }
            }
        }

        res
    }
}

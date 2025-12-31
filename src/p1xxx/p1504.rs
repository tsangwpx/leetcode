// Problem 1504
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        #[derive(Debug, Clone, Copy, Default)]
        struct Item {
            index: i32,
            height: i32,
            acc: i32,
        }

        let mut heights = vec![0; n];

        let mut count = 0;

        for row in mat.iter() {
            assert!(row.len() == n);

            let mut stack: Vec<Item> = vec![Item {
                index: -1,
                height: -1,
                acc: 0,
            }];

            for (idx, item) in row.iter().copied().enumerate() {
                if item == 0 {
                    heights[idx] = 0;
                } else {
                    heights[idx] += 1;
                }

                let h = heights[idx];

                while let Some(_) = stack.pop_if(|s| s.height >= h) {}

                let &Item {
                    index: last_idx,
                    acc: last_acc,
                    ..
                } = stack.last().unwrap();

                stack.push(Item {
                    index: idx as i32,
                    height: h,
                    acc: last_acc + (idx as i32 - last_idx) * h,
                });

                count += stack.last().unwrap().acc;
            }
        }

        count
    }
}

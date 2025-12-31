// Problem 3495
impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        #[derive(Debug, Clone, Copy)]
        struct Row {
            limit: i64,
            operations: i64,
        }

        let table = {
            let mut table = vec![];
            // const LIMIT: i64 = 1i64.pow(9);

            for k in 0..16 {
                let mask = 1i64 << (k * 2);

                table.push((mask, k));
            }

            table
        };

        let mut sum = 0i64;

        for query in queries.iter() {
            let &[left, right] = query.as_slice() else {
                panic!("Bad input");
            };
            let left = i64::from(left);
            let right = i64::from(right);

            let mut div_count = 0i64;
            let mut max_ops = 0;

            for (i, (stop, ops)) in table.iter().copied().enumerate().skip(1) {
                let start = table[i - 1].0;

                // println!("{} {} {} {} {} ", left, right, start, stop, div_count);

                if start <= left && right < stop {
                    div_count += (right + 1 - left) * ops;
                    max_ops = max_ops.max(ops);
                } else if left <= start && stop <= right {
                    div_count += (stop - start) * ops;
                    max_ops = max_ops.max(ops);
                } else if start <= left && left < stop {
                    div_count += (stop - left) * ops;
                    max_ops = max_ops.max(ops);
                } else if start <= right && right < stop {
                    div_count += (right + 1 - start) * ops;
                    max_ops = max_ops.max(ops);
                }
            }

            let opcount = (div_count / 2 + (div_count & 1)).max(max_ops);
            // println!("{left} {right} {div_count} {opcount}");
            sum += opcount;
        }

        sum
    }
}

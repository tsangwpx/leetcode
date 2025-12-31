// Problem 282
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        /** Backtracking
         */
        let mut res = vec![];

        /**
         * `prev` is the value added to `partial` in the last operation.
         *
         * For multiplication operation, we need to subtract `prev` from `partial`,
         * and add it back with a multiplicative factor.
         *
         */
        fn dfs(
            res: &mut Vec<String>,
            expr: &mut String,
            num: &str,
            start: usize,
            target: i64,
            partial: i64,
            prev: i64,
        ) {
            if start == num.len() {
                if partial == target {
                    res.push(expr.clone());
                }
                return;
            }

            for stop in (start + 1)..=num.len() {
                let expr_len = expr.len();
                let number = &num[start..stop];
                if number.len() >= 2 && number.chars().nth(0).unwrap() == '0' {
                    // skip number with leading zeros
                    continue;
                }

                let value = number.parse::<i64>().unwrap();

                if start == 0 {
                    expr.push_str(&num[start..stop]);
                    dfs(res, expr, num, stop, target, partial + value, value);
                    expr.truncate(expr_len);
                } else {
                    expr.push('+');
                    expr.push_str(number);
                    dfs(res, expr, num, stop, target, partial + value, value);
                    expr.truncate(expr_len);

                    expr.push('-');
                    expr.push_str(number);
                    dfs(res, expr, num, stop, target, partial - value, -value);
                    expr.truncate(expr_len);

                    expr.push('*');
                    expr.push_str(number);
                    dfs(
                        res,
                        expr,
                        num,
                        stop,
                        target,
                        partial - prev + prev * value,
                        prev * value,
                    );
                    expr.truncate(expr_len);
                }
            }
        }

        dfs(
            &mut res,
            &mut String::new(),
            &num,
            0,
            i64::from(target),
            0,
            0,
        );

        res
    }
}

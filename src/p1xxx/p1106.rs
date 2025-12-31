// Problem 1106
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        fn eval<'a>(mut expr: &'a [u8]) -> (bool, &'a [u8]) {
            let first = expr.first().copied().unwrap();

            match first {
                b't' => (true, &expr[1..]),
                b'f' => (false, &expr[1..]),
                b'!' => {
                    expr = &expr[2..]; // skip "!("

                    let result;
                    (result, expr) = eval(expr);

                    expr = &expr[1..]; // skip ")"
                    (!result, expr)
                }
                b'|' | b'&' => {
                    expr = &expr[2..]; // skip "|(" or "&("

                    let mut result;
                    (result, expr) = eval(expr);

                    while expr.get(0).copied() == Some(b',') {
                        expr = &expr[1..]; // skip ","

                        let value;
                        (value, expr) = eval(expr);

                        result = match first {
                            b'|' => result || value,
                            b'&' => result && value,
                            _ => unreachable!(),
                        }
                    }

                    expr = &expr[1..]; // skip ")"

                    (result, expr)
                }
                _ => panic!("Bad character: {}", first),
            }
        }

        eval(expression.as_bytes()).0
    }

    pub fn parse_bool_expr2(expression: String) -> bool {
        fn eval(expr: &[u8]) -> (bool, usize) {
            let len = expr.len();
            assert!(expr.len() >= 1);

            let first = expr.first().copied().unwrap();

            match first {
                b't' => (true, 1),
                b'f' => (false, 1),
                b'!' => {
                    assert!(expr.len() >= 4);
                    let start = 2;
                    let (val, size) = eval(&expr[start..]);
                    let size = start + size + 1;
                    (!val, size)
                }
                b'|' | b'&' => {
                    let mut idx = 2;
                    let (mut result, size) = eval(&expr[idx..]);
                    idx += size;

                    while expr.get(idx).copied().unwrap() == b',' {
                        idx += 1; // skip comma
                        let (val, size) = eval(&expr[idx..]);
                        idx += size;
                        result = match first {
                            b'|' => result || val,
                            b'&' => result && val,
                            _ => unreachable!(),
                        }
                    }

                    idx += 1; // skip close bracket

                    (result, idx)
                }
                _ => panic!("Bad character: {}", first),
            }
        }

        eval(expression.as_bytes()).0
    }
}

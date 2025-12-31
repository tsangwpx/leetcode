// Problem 386
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut out = Vec::with_capacity(n as usize);

        let mut num = 1;
        out.push(num);

        for _ in 1..n {
            // 1. try to add a trailing zero
            if num * 10 <= n {
                num *= 10;
                out.push(num);
                continue;
            }

            // 2. try to bump the number, if the last digit is not 9
            if num + 1 <= n && (num % 10) != 9 {
                num += 1;
                out.push(num);
                continue;
            }

            // 3. keep shrinking the number, until one is added
            loop {
                num /= 10;
                if num % 10 == 9 {
                    // cant add 1 to number with trailing 9
                    continue;
                }
                if num == 0 {
                    // ONE have already been added. exiting.
                    break;
                }

                if num + 1 <= n {
                    num += 1;
                    out.push(num);
                    break;
                }
            }

            if num == 0 {
                break;
            }
        }

        out
    }

    pub fn lexical_order2(n: i32) -> Vec<i32> {
        let mut out = vec![];

        fn dfs(out: &mut Vec<i32>, base: i32, n: i32) {
            if base > n {
                return;
            }

            out.push(base);

            let base = base * 10;

            for i in 0..=9 {
                dfs(out, base + i, n);
            }
        }

        for i in 1..=9 {
            dfs(&mut out, i, n);
        }

        out
    }
}

// Problem 77
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        assert!(k >= 1);
        assert!(n >= 1 && n >= k);

        fn combination(n: i32, r: i32) -> usize {
            let n = n as u64;
            let r = r as u64;
            let r = r.max(n - r);
            let mut c = 1u64;
            for i in r + 1..=n {
                c *= i as u64;
            }
            for i in 2u64..=(n - r) {
                c /= i;
            }
            c as usize
        }

        let cap = combination(n, k);
        let len = k as usize;
        let mut combinations = vec![vec![0i32; len]; cap];

        let mut idx = 0;

        let mut numbers = vec![0; len];
        let mut stack = Vec::with_capacity(len + 1);
        stack.push((0usize, 1i32));

        while let Some((pos, num)) = stack.pop() {
            numbers[pos] = num;

            if num + 1 <= n + 1 - (len - pos) as i32 {
                stack.push((pos, num + 1));
            }

            if pos + 1 == len {
                combinations[idx].copy_from_slice(&numbers);
                idx += 1;
            } else {
                stack.push((pos + 1, num + 1));
            }
        }

        combinations
    }

    pub fn combine2(n: i32, k: i32) -> Vec<Vec<i32>> {
        assert!(k >= 1);
        assert!(n >= 1 && n >= k);

        fn combination(n: i32, r: i32) -> usize {
            let n = n as u64;
            let r = r as u64;
            let r = r.max(n - r);
            let mut c = 1u64;
            for i in r + 1..=n {
                c *= i as u64;
            }
            for i in 2u64..=(n - r) {
                c /= i;
            }
            c as usize
        }

        let cap = combination(n, k);
        println!("{}", cap);
        let mut combinations = Vec::with_capacity(cap);
        // let mut combinations = Vec::new();

        // say, n = 10, k = 3
        // numbers is initialized to [1,2,3]

        let len = k as usize;
        let mut numbers = vec![0; len];

        #[inline]
        fn recurse(
            res: &mut Vec<Vec<i32>>,
            numbers: &mut Vec<i32>,
            pos: usize,
            start: i32,
            stop: i32,
        ) {
            if pos >= numbers.len() {
                res.push(numbers.clone());
                return;
            }

            let slots = numbers.len() - pos;
            let adjusted_stop = stop + 1 - slots as i32;
            for num in start..=adjusted_stop {
                numbers[pos] = num;
                recurse(res, numbers, pos + 1, num + 1, stop);
            }
        }

        recurse(&mut combinations, &mut numbers, 0, 1, n);

        combinations
    }
}

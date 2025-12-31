// Problem 2081
impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        // very slow!!

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let k = k as i64;

        fn get_kbuf(value: i64, k: i64) -> Vec<i8> {
            let mut buf = vec![];
            let mut rem = value;

            while rem > 0 {
                let (q, r) = (rem / k, rem % k);
                buf.push(r as i8);
                rem = q;
            }

            buf.reverse();

            buf
        }

        let mut heap = BinaryHeap::new();

        for i in 1..k {
            heap.push((Reverse(i), Some((i, i, k))));
            heap.push((Reverse(i * k + i), None));
        }

        let mut found = 0;
        let mut sum = 0i64;
        let mut dbuf = vec![];

        while let Some((Reverse(value), info)) = heap.pop() {
            {
                dbuf.clear();

                let mut rem = value;

                while rem > 0 {
                    let (q, r) = (rem / 10, rem % 10);
                    dbuf.push(r as i8);
                    rem = q;
                }
            }

            if (0..dbuf.len() / 2).all(|s| dbuf[s] == dbuf[dbuf.len() - s - 1]) {
                // println!("Found {value} {dbuf:?}");
                sum += value;
                found += 1;
            }

            if found >= n {
                break;
            }

            if let Some((prefix, suffix, scale)) = info {
                for digit in 0..k {
                    let prefix2 = prefix * k + digit;
                    let suffix2 = scale * digit + suffix;
                    let scale2 = scale * k;

                    let odd_value = prefix2 * scale + suffix;
                    let even_value = prefix2 * scale2 + suffix2;

                    // println!(
                    //     "{} {:?} {:?} {:?} {:?} {:?}",
                    //     digit,
                    //     get_kbuf(prefix2, k),
                    //     get_kbuf(suffix2, k),
                    //     get_kbuf(scale, k),
                    //     get_kbuf(odd_value, k),
                    //     get_kbuf(even_value, k),
                    // );

                    heap.push((Reverse(odd_value), Some((prefix2, suffix2, scale2))));
                    heap.push((Reverse(even_value), None));
                }
            }
        }

        sum
    }
}

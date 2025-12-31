// Problem 1399
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        // use std::collections::HashMap;

        // // let mut table = HashMap::<i32, i32>::new();

        // the maximum sum of digits under 10**4 is 9 + 9 + 9 + 9 = 36
        // so 37 is enough
        let mut table = [0i32; 37];

        for x0 in 0..10 {
            let y0 = x0 * 1000;
            if y0 > n {
                break;
            }

            for x1 in 0..10 {
                let y1 = y0 + x1 * 100;
                if y1 > n {
                    break;
                }

                for x2 in 0..10 {
                    let y2 = y1 + x2 * 10;
                    if y2 > n {
                        break;
                    }

                    for x3 in 0..10 {
                        let y3 = y2 + x3;
                        if y3 > n {
                            break;
                        }

                        let sum = x0 + x1 + x2 + x3;
                        table[sum as usize] += 1;

                        // *table.entry(sum).or_default() += 1;
                    }
                }
            }
        }

        // make sure 0 is excluded
        table[0] = 0;

        // println!("{:?}", table);

        let mut max_group = 0;
        let mut max_count = 0;

        for size in table.iter().copied() {
            if size > max_group {
                max_group = size;
                max_count = 1;
            } else if size == max_group {
                max_count += 1;
            }
        }
        max_count
    }
}

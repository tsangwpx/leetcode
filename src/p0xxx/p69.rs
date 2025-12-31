impl Solution {
    // Problem 69
    pub fn my_sqrt(x: i32) -> i32 {
        match x {
            0 => return 0,
            1 => return 1,
            _ => {}
        }

        let x = x as i64;
        let mut a = 0 as i64;
        let mut b = x as i64;

        use std::cmp::Ordering;

        while a < b {
            let mid = (a + b) / 2;
            let squ = mid * mid;

            match x.cmp(&squ) {
                Ordering::Less => b = mid,
                Ordering::Greater => {
                    if a == mid {
                        break;
                    }
                    a = mid
                }
                Ordering::Equal => break,
            }
        }

        ((a + b) / 2) as i32
    }
}

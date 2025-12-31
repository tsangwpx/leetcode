unsafe fn guess(num: i32) -> i32 {
    0
}

// Problem 374
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        use std::cmp::Ordering;
        let mut left = 1i64;
        let mut right = n as i64;

        while left < right {
            // println!("{}", left);
            let mid = (left + right) / 2;
            match guess(mid as i32).cmp(&0) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => right = mid - 1,
                Ordering::Greater => left = mid + 1,
            }
        }

        left as i32
    }
}

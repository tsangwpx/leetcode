impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use ::std::cmp::{Ordering, max, min};

        let total = nums1.len() + nums2.len();
        let even = total % 2 == 0;
        let mut remaining = total / 2;
        if even && remaining > 0 {
            remaining -= 1;
        }
        let mut nums1 = &nums1[..];
        let mut nums2 = &nums2[..];

        while remaining > 0 && nums1.len() > 0 && nums2.len() > 0 {
            // println!("num1 {:?} {} {}", nums1, remaining, even);
            // println!("num2 {:?}", nums2);
            let m1 = nums1.len() / 2;
            let v1 = nums1[m1];
            let m2 = nums2.len() / 2;
            let v2 = nums2[m2];

            // The sequence with larger  mid value is called "large"; "small" otherwise.
            // large_left and small_right are unknown region that ordering cannot be determined by comparing mid value
            // Consider size of large_left, try to remove as many as possible item from small_left
            // prefixes "s" and "l" mean "small" and "large" sequences
            // suffixes "a", "m", and "b" mean start, mid, and stop of sequences
            // small:   [sa, sm]        (sm, sb)
            //          ^ small_left    ^ small_right
            // large:   [la, lm)        [lm, lb)
            //          ^ large_left    ^ large_right
            let progress;
            if v1 <= v2 {
                progress = min(m1 + 1, remaining.saturating_sub(m2));
                if progress > 0 {
                    remaining -= progress;
                    nums1 = &nums1[progress..];
                }
            } else {
                progress = min(m2 + 1, remaining.saturating_sub(m1));
                if progress > 0 {
                    remaining -= progress;
                    nums2 = &nums2[progress..];
                }
            }

            if progress == 0 {
                // in order to make progress
                // compare head items of both sequences
                // bisect the sequence of smaller head item with the larger head item
                // equality comparison is also counted into progress
                let head1 = nums1[0];
                let head2 = nums2[0];
                if head1 <= head2 {
                    let progress = nums1
                        .binary_search_by(|&s| s.cmp(&head2).then(Ordering::Less))
                        .unwrap_or_else(|s| s)
                        .min(remaining);
                    remaining -= progress;
                    nums1 = &nums1[progress..];
                } else {
                    let progress = nums2
                        .binary_search_by(|&s| s.cmp(&head1).then(Ordering::Less))
                        .unwrap_or_else(|s| s)
                        .min(remaining);
                    remaining -= progress;
                    nums2 = &nums2[progress..];
                }
            }
        }

        // println!("num1 {:?} {} {}", nums1, remaining, even);
        // println!("num2 {:?}", nums2);
        if remaining > 0 {
            // One of the sequence have been exhausted
            if nums1.len() == 0 {
                nums2 = &nums2[remaining..];
            } else if nums2.len() == 0 {
                nums1 = &nums1[remaining..];
            } else {
                unreachable!("{} {} {}", nums1.len(), nums2.len(), remaining);
            }
            remaining = 0;
        }

        // remaining ensure there are enough valid items in each sequence
        // so i32::MAX can never be contributed to the final result
        let mut work = [i32::MAX; 4];
        let mut idx = 0;
        for &n in &nums1[..min(2, nums1.len())] {
            work[idx] = n;
            idx += 1;
        }
        for &n in &nums2[..min(2, nums2.len())] {
            work[idx] = n;
            idx += 1;
        }
        work.sort_unstable();
        if even {
            (f64::from(work[0]) + f64::from(work[1])) / 2.0
        } else {
            f64::from(work[0])
        }
    }

    pub fn find_median_sorted_arrays3(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use ::std::cmp::{Ordering, max, min};

        let total = nums1.len() + nums2.len();
        let even = total % 2 == 0;
        let mut rem_left = total / 2;
        if even && rem_left > 0 {
            rem_left -= 1;
        }
        let mut rem_right = rem_left;

        let mut a1 = 0;
        let mut b1 = nums1.len();
        let mut a2 = 0;
        let mut b2 = nums2.len();

        // /// Count small items in the "large" sequence not greater than val
        // #[inline(always)]
        // fn bisect_large_left(nums: &Vec<i32>, a: usize, b: usize, val: i32) -> usize {
        //     let m = (a + b) / 2;
        //     // SAFETY: a < b must be held in the while loop, thus a <= m
        //     let slice = unsafe { nums.get_unchecked(a..m) };
        //     slice.binary_search_by(|&s| s.cmp(&val).then(Ordering::Greater))
        //         .unwrap_or_else(|s| s)
        // }
        //
        // /// Count large items in the "small" sequence not less than val
        // #[inline(always)]
        // fn bisect_small_right(nums: &Vec<i32>, a: usize, b: usize, val: i32) -> usize {
        //     let m = (a + b) / 2;
        //     // SAFETY: a < b must be held in the while loop, thus m + 1 <= b
        //     let slice = unsafe { nums.get_unchecked(m + 1..b) };
        //     let r = slice.binary_search_by(|&s| s.cmp(&val).then(Ordering::Less))
        //         .unwrap_or_else(|s| s);
        //     slice.len() - r
        // }

        macro_rules! get_unchecked {
            ($vec:expr, $index:expr) => {
                *unsafe { $vec.get_unchecked($index) }
            };
        }

        #[inline(always)]
        fn update(
            small: &Vec<i32>,
            sa: &mut usize,
            sb: &mut usize,
            large: &Vec<i32>,
            la: &mut usize,
            lb: &mut usize,
            rem_left: &mut usize,
            rem_right: &mut usize,
        ) {
            // small:   [sa, sm]        (sm, sb)
            //          ^ small_left    ^ small_right
            // large:   [la, lm)        [lm, lb)
            //          ^ large_left    ^ large_right
            let sm = (*sa + *sb) / 2;
            let lm = (*la + *lb) / 2;

            let mut small_left = min(sm + 1 - *sa, rem_left.saturating_sub(lm - *la));
            if small_left < *rem_left
                && *sa + small_left <= sm
                && get_unchecked!(small, *sa + small_left) <= get_unchecked!(large, *la)
            {
                small_left += 1;
            }
            *rem_left -= small_left;
            *sa += small_left;

            if small_left == 0
                && *rem_left > 0
                && *la < lm
                && get_unchecked!(large, *la) <= get_unchecked!(small, sm)
            {
                *rem_left -= 1;
                *la += 1;
            }

            let mut large_right = min(*lb - lm, rem_right.saturating_sub(*sb - sm - 1));
            if large_right < *rem_right && lm + large_right < *lb {
                assert!(sm + 1 < *sb);
                if get_unchecked!(large, *lb - large_right - 1) >= get_unchecked!(small, *sb - 1) {
                    large_right += 1;
                }
            }
            *rem_right -= large_right;
            *lb -= large_right;

            if large_right == 0
                && *rem_right > 0
                && sm + 1 < *sb
                && get_unchecked!(small, *sb - 1) >= get_unchecked!(large, sm)
            {
                *rem_right -= 1;
                *sb -= 1;
            }
        }

        while a1 < b1 && a2 < b2 && (rem_left > 0 || rem_right > 0) {
            // println!("1={} {}; 2={} {}; rem={} {}", a1, b1, a2, b2, rem_left, rem_right);

            let m1 = (a1 + b1) / 2;
            let m2 = (a2 + b2) / 2;
            // SAFETY: 0 <= a < b <= nums.len() must be held.
            let v1 = nums1[m1];
            let v2 = nums2[m2];

            println!(
                "cmp {:?} nums1={} {} {} {} nums2={} {} {} {} rem={} {}",
                v1.cmp(&v2),
                a1,
                b1,
                m1,
                v1,
                a2,
                b2,
                m2,
                v2,
                rem_left,
                rem_right
            );

            if v1 <= v2 {
                update(
                    &nums1,
                    &mut a1,
                    &mut b1,
                    &nums2,
                    &mut a2,
                    &mut b2,
                    &mut rem_left,
                    &mut rem_right,
                );
            } else {
                update(
                    &nums2,
                    &mut a2,
                    &mut b2,
                    &nums1,
                    &mut a1,
                    &mut b1,
                    &mut rem_left,
                    &mut rem_right,
                );
            }
        }

        if rem_left > 0 || rem_right > 0 {
            // One of the sequence have been exhausted
            if a1 == b1 {
                a2 += rem_left;
                b2 -= rem_right;
            } else if a2 == b2 {
                a1 += rem_left;
                b1 -= rem_right;
            } else {
                unreachable!("{} {} {} {}", a1, b1, a2, b2);
            }
            rem_left = 0;
            rem_right = 0;
        }

        // println!("nums1: {:?} {} {}", nums1, a1, b1);
        // println!("nums2: {:?} {} {}", nums2, a2, b2);
        // println!("rem: {} {} even {}", rem_left, rem_right, even);
        // assert!((even && (b1 - a1 + b2 - a2) == 2) || (!even && (b1 - a1 + b2 - a2) == 1));

        let mut median: f64 = 0.0;

        while a1 < b1 {
            median += f64::from(nums1[a1]);
            a1 += 1;
        }

        while a2 < b2 {
            median += f64::from(nums2[a2]);
            a2 += 1;
        }

        if even { median / 2.0 } else { median }
    }

    pub fn find_median_sorted_arrays2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use ::std::cmp::{Ordering, max, min};

        let total = nums1.len() + nums2.len();
        let even = total % 2 == 0;
        let mut rem_left = total / 2;
        if even && rem_left > 0 {
            rem_left -= 1;
        }
        let mut rem_right = rem_left;

        let mut a1 = 0;
        let mut b1 = nums1.len();
        let mut a2 = 0;
        let mut b2 = nums2.len();

        #[inline(always)]
        fn seq_bounds(
            rem_left: usize,
            rem_right: usize,
            sa: usize,
            sb: usize,
            la: usize,
            lb: usize,
        ) -> (usize, usize, usize, usize) {
            // Small:   [start, mid] (mid, stop)
            //              ^ go left   ^ su
            // Large:   [start, mid) [mid, stop)
            //              ^ lu        ^ go right
            let sm = (sa + sb) / 2;
            let su = sb - sm + 1;
            let lm = (la + lb) / 2;
            let lu = lm - la;

            // left side: sa_pos <= sa_stop
            // sa_pos:  the left endpoint to compare with smallest item in peer seq
            // sa_stop: the left endpoint limit
            let sa_stop = min(sm + 1, sa + rem_left);
            let sa_pos = min(sa_stop, sa + rem_left.saturating_sub(lu));

            // right side: lb_stop <= lb_pos
            // lb_pos:  the right endpoint (exclusive in large seq) to start comparing with the largest item in peer seq
            // lb_stop: the right endpoint (exclusive in large seq) limit
            let lb_stop = max(lm, lb.saturating_sub(rem_right));
            let lb_pos = max(lb_stop, lb.saturating_sub(rem_right.saturating_sub(su)));

            // sa0 can be sa1
            // lb0 can be lb1
            (sa_pos, sa_stop, lb_pos, lb_stop)
        }

        while a1 < b1 && a2 < b2 && (rem_left > 0 || rem_right > 0) {
            // println!("1={} {}; 2={} {}; rem={} {}", a1, b1, a2, b2, rem_left, rem_right);

            let m1 = (a1 + b1) / 2;
            let v1 = nums1[m1];
            let m2 = (a2 + b2) / 2;
            let v2 = nums2[m2];

            let cmp = v1.cmp(&v2);
            let mut eq = cmp == Ordering::Equal;

            if cmp.is_ge() {
                // v1 >= v2
                let (mut sa, sa_stop, mut lb, lb_stop) =
                    seq_bounds(rem_left, rem_right, a2, b2, a1, b1);
                // println!("ge sa={} {}; lb={} {}", sa, sa_stop, lb, lb_stop);
                while sa < sa_stop && nums2[sa] <= nums1[a1] {
                    sa += 1;
                }

                while lb > lb_stop && nums1[lb - 1] >= nums2[b2 - 1] {
                    lb -= 1;
                }

                eq = eq && a2 == sa && b1 == lb;
                rem_left -= sa - a2;
                rem_right -= b1 - lb;
                a2 = sa;
                b1 = lb;

                // remove the leftmost element in the larger sequence if less than smaller mid value
                if rem_left > 0 && a1 < b1 && nums1[a1] < v2 {
                    a1 += 1;
                    rem_left -= 1;
                    eq = false;
                }
            }

            if cmp.is_lt() || eq {
                // v1 <= v2
                let (mut sa, sa_stop, mut lb, lb_stop) =
                    seq_bounds(rem_left, rem_right, a1, b1, a2, b2);
                // println!("le sa={} {}; lb={} {}", sa, sa_stop, lb, lb_stop);
                while sa < sa_stop && nums1[sa] <= nums2[a2] {
                    sa += 1;
                }

                while lb > lb_stop && nums2[lb - 1] >= nums1[b1 - 1] {
                    lb -= 1;
                }

                rem_left -= sa - a1;
                rem_right -= b2 - lb;
                a1 = sa;
                b2 = lb;

                // remove the leftmost element in the larger sequence if less than smaller mid value
                if rem_left > 0 && a2 < b2 && nums2[a2] < v1 {
                    a2 += 1;
                    rem_left -= 1;
                }
            }
        }

        if rem_left > 0 || rem_right > 0 {
            // One of the sequence have been exhausted
            if a1 == b1 {
                a2 += rem_left;
                b2 -= rem_right;
            } else if a2 == b2 {
                a1 += rem_left;
                b1 -= rem_right;
            } else {
                unreachable!();
            }
            rem_left = 0;
            rem_right = 0;
        }

        // println!("nums1: {:?} {} {}", nums1, a1, b1);
        // println!("nums2: {:?} {} {}", nums2, a2, b2);
        // println!("rem: {} {} even {}", rem_left, rem_right, even);
        // assert!((even && (b1 - a1 + b2 - a2) == 2) || (!even && (b1 - a2 + b2 - a2) == 1));

        let mut median: f64 = 0.0;

        while a1 < b1 {
            median += f64::from(nums1[a1]);
            a1 += 1;
        }

        while a2 < b2 {
            median += f64::from(nums2[a2]);
            a2 += 1;
        }

        if even { median / 2.0 } else { median }
    }
}

fn main() {
    println!("Hello World");
}

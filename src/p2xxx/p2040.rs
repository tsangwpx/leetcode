// Problem 2040
impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        const ITEM_MIN: i64 = -10i64.pow(5);
        const ITEM_MAX: i64 = 10i64.pow(5);
        const PROD_MIN: i64 = ITEM_MIN * ITEM_MAX;
        const PROD_MAX: i64 = ITEM_MAX * ITEM_MAX;

        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let mut left = PROD_MIN;
        let mut right = PROD_MAX;

        const DEBUG: bool = true;

        #[inline]
        fn count_le(nums1: &Vec<i32>, nums2: &Vec<i32>, target: i64) -> i64 {
            if DEBUG {
                println!("target {}", target);
            }

            let mut count = 0;
            for item in nums1.iter().copied() {
                let item = item as i64;

                match item.cmp(&0) {
                    std::cmp::Ordering::Greater => {
                        // item > 0
                        // count good item from the left
                        let idx2 = nums2.partition_point(|&s| i64::from(s) * item <= target);
                        count += idx2 as i64;

                        if DEBUG {
                            println!("gt {} {}", item, idx2);
                        }
                    }
                    std::cmp::Ordering::Less => {
                        // if item < 0, (target / item) is of opposite sign
                        // count bad item from the left
                        // good items are obtained by subtracting bad items from total
                        let idx2 = nums2.partition_point(|&s| i64::from(s) * item > target);

                        count += (nums2.len() - idx2) as i64;

                        if DEBUG {
                            println!("lt {} {}", item, (nums2.len() - idx2));
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        if target >= 0 {
                            count += nums2.len() as i64;
                        }

                        if DEBUG {
                            println!("eq {} {} {}", item, target, nums2.len());
                        }
                    }
                }
            }

            count
        }

        while left < right {
            let target = (right - left) / 2 + left;

            let count = count_le(&nums1, &nums2, target);

            if DEBUG {
                println!("bisect {} {} {} {}", target, count, left, right);
            }

            if count < k {
                left = target + 1;
            } else {
                right = target;
            }
        }

        left
    }
}

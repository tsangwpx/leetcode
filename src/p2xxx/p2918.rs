// Problem 2918
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        #[inline]
        fn reduce(nums: &Vec<i32>) -> (i64, i64) {
            nums.iter().copied().fold((0, 0), |(count, sum), item| {
                if item == 0 {
                    (count + 1, sum)
                } else {
                    (count, sum + i64::from(item))
                }
            })
        }

        let (mut free1, mut sum1) = reduce(&nums1);
        let (mut free2, mut sum2) = reduce(&nums2);

        // println!("{} {}", free1, sum1);
        // println!("{} {}", free2, sum2);

        if free1 == 0 && free2 == 0 {
            return if sum1 != sum2 { -1 } else { sum1 as i64 };
        }

        if free1 > 0 && free2 > 0 {
            return (free1 + sum1).max(free2 + sum2);
        }

        if free1 == 0 {
            (free1, free2) = (free2, free1);
            (sum1, sum2) = (sum2, sum1);
        }

        let min1 = free1 + sum1;
        // println!("{} {}", free1, sum1);
        // println!("{} {}", free2, sum2);

        if min1 > sum2 { -1 } else { min1.max(sum2) }
    }
}

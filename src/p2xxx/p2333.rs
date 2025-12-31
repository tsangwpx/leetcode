impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        assert_eq!(nums1.len(), nums2.len());
        assert!(1 <= nums1.len());

        let mut nums3: Vec<_> = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&a, &b)| (a - b).abs())
            .collect();

        nums3.sort_by(|&a, &b| b.cmp(&a));

        let mut k = k1 + k2;
        let mut ans = 0i64;
        let mut i = 0;
        let mut largest = nums3[0];
        let mut multiple = 1;
        i += 1;

        while i < nums3.len() {
            let number = nums3[i];
            assert!(largest >= number);
            println!(
                "i {} largest={} x{}, number={}",
                i, largest, multiple, number
            );
            let dk = (largest - number) * multiple;

            if k < dk {
                // not enough k to reduce the largest difference to the second largest one
                break;
            }

            k -= dk;
            i += 1;
            largest = number;
            multiple += 1;
        }

        if k > 0 {
            // reduce largest difference as much as possible
            let mut q = k / multiple;
            let r = k % multiple;

            largest = std::cmp::max(0, largest - q);

            if largest >= 1 {
                ans += i64::from(largest).pow(2) * i64::from(multiple - r);

                if largest >= 2 {
                    ans += i64::from(largest - 1).pow(2) * i64::from(r);
                }
            }
        } else {
            ans += i64::from(largest).pow(2) * i64::from(multiple);
        }

        while i < nums3.len() {
            ans += i64::from(nums3[i]).pow(2);
            i += 1;
        }

        ans
    }
}

fn main() {
    Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0);
    println!("Hello World");
}

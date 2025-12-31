// Problem 2179
impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        // https://leetcode.com/problems/count-good-triplets-in-an-array/solutions/1783180/python-2-fenwick-trees-solution-explained/

        let n = nums1.len();

        fn from_permuted(nums: &Vec<i32>) -> Vec<i32> {
            let mut indices = vec![0; nums.len()];

            for (idx, number) in nums.iter().copied().enumerate() {
                let number = number as usize;
                if number < indices.len() {
                    indices[number] = idx as i32;
                }
            }

            indices
        }

        struct BIT {
            data: Vec<i64>,
        }

        impl BIT {
            fn new(n: usize) -> Self {
                Self {
                    data: vec![0; n + 1],
                }
            }

            fn update(&mut self, idx: usize, delta: i64) {
                let mut idx = idx;
                while idx < self.data.len() {
                    self.data[idx] += delta;
                    idx += idx & ((!idx).wrapping_add(1));
                }
            }

            fn query(&self, mut idx: usize) -> i64 {
                let mut res = 0;
                while idx > 0 {
                    res += self.data[idx];
                    idx -= idx & ((!idx).wrapping_add(1));
                }

                res
            }
        }

        let indices1 = from_permuted(&nums1);
        let mut bit1 = BIT::new(n);
        let mut bit2 = BIT::new(n);
        let mut ans = 0;

        for idx2 in 0..n {
            let item = nums2[idx2];
            let idx1 = indices1[item as usize] as usize;

            ans += bit2.query(idx1);
            bit1.update(idx1 + 1, 1);
            let less = bit1.query(idx1);
            bit2.update(idx1 + 1, less);
        }

        ans
    }
}

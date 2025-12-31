impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        use ::std::collections::HashMap;

        // sort indices by values
        let mut indices: Vec<usize> = (0..nums.len()).collect();
        indices.sort_by_key(|&s| nums[s]);

        // form a virtual sorted nums by indices which does not need to exist
        // sorted_unique_nums = sorted(set(nums))

        // build mapping number -> their index
        let mut table = HashMap::<i32, usize>::with_capacity(nums.len());
        indices.into_iter().enumerate().for_each(|(i, s)| {
            table.entry(nums[s]).or_insert(i);
        });

        // binary index tree
        // prefix sums
        let mut tree = vec![0i32; nums.len() + 1];
        let mut counts = vec![0i32; nums.len()];

        #[inline(always)]
        fn sum_before(tree: &mut [i32], mut idx: usize) -> i32 {
            let mut sum = 0;
            while idx > 0 {
                sum += tree[idx];
                let s = idx as isize;
                idx -= (s & (-s)) as usize;
            }
            sum
        }

        #[inline(always)]
        fn update(tree: &mut [i32], mut idx: usize, val: i32) {
            idx += 1;
            while idx < tree.len() {
                tree[idx] += val;
                let s = idx as isize;
                idx += (s & (-s)) as usize;
            }
        }

        // println!("{:?}", table);

        for i in (0..nums.len()).rev() {
            let number = nums[i];
            let idx = *table.get(&number).unwrap();

            // sum values between 0 up to idx (exclusive)
            counts[i] = sum_before(&mut tree, idx);
            // update values between 0 up to idx (exclusive)
            update(&mut tree, idx, 1);

            // println!("{}: num={} idx={}, tree={:?}", i, number, idx, tree);
        }

        counts
    }

    pub fn count_smaller2(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![0i32; nums.len()];
        let mut indices = (0..nums.len()).collect::<Vec<usize>>();
        let mut indices2 = indices.clone();

        fn mergesort(
            nums: &[i32],
            counts: &mut [i32],
            indices: &mut [usize],
            indices2: &mut [usize],
            start: usize,
            stop: usize,
        ) {
            let count = stop - start;
            if count <= 1 {
                return;
            }

            let mid = (start + stop) / 2;

            mergesort(nums, counts, indices2, indices, start, mid);
            mergesort(nums, counts, indices2, indices, mid, stop);

            let (mut i, i_stop) = (start, mid);
            let (mut j, j_stop) = (mid, stop);
            let mut k = start;
            let mut smaller_count = 0;

            while i < i_stop && j < j_stop {
                let a = nums[indices2[i]];
                let b = nums[indices2[j]];

                if a <= b {
                    indices[k] = indices2[i];
                    counts[indices2[i]] += smaller_count;
                    i += 1;
                    k += 1;
                } else {
                    indices[k] = indices2[j];
                    smaller_count += 1;
                    j += 1;
                    k += 1;
                }
            }

            for i in i..i_stop {
                indices[k] = indices2[i];
                counts[indices2[i]] += smaller_count;
                k += 1;
            }

            for j in j..j_stop {
                indices[k] = indices2[j];
                k += 1;
            }
        }

        mergesort(
            &nums,
            &mut counts,
            &mut indices,
            &mut indices2,
            0,
            nums.len(),
        );

        counts
    }
}

fn main() {
    Solution::count_smaller(vec![5, 2, 6, 1]);

    Solution::count_smaller(vec![-1]);

    Solution::count_smaller(vec![-1, -1]);
    println!("Hello World");
}

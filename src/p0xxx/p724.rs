// Problem 724
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);

        let mut sfxsums = vec![0; nums.len()];
        sfxsums[nums.len() - 1] = *nums.last().unwrap();

        nums.iter()
            .enumerate()
            .rev()
            .skip(1)
            .for_each(|(i, &number)| {
                sfxsums[i] = sfxsums[i + 1] + number;
            });

        // println!("{:?}", sfxsums);

        let mut acc = 0;

        for i in 0..nums.len() - 1 {
            if acc == sfxsums[i + 1] {
                return i as i32;
            }
            acc += nums[i];
        }

        if acc == 0 {
            // pivot is the (len - 1)-th position
            return nums.len() as i32 - 1;
        }

        -1
    }
}

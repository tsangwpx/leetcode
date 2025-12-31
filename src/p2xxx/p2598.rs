// Problem 2598
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut buckets = vec![0; value as usize];

        for item in nums.iter().copied() {
            let mut rem = (item % value);
            if rem < 0 {
                rem += value;
            }

            buckets[rem as usize] += 1;

            println!("{} {}", item, rem);
        }

        let (idx, count) = buckets
            .iter()
            .copied()
            .enumerate()
            .min_by_key(|&(idx, cnt)| (cnt, idx))
            .unwrap();

        count * value + idx as i32
    }
}

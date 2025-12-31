// Problem 1497
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut buckets = vec![0; k as usize];
        for item in arr.iter().copied() {
            let rem = item.rem_euclid(k);
            buckets[rem as usize] += 1;
        }

        if buckets[0] % 2 != 0 {
            // the first remainder (i.e. zero) must be paired with itself
            return false;
        }

        if k % 2 == 0 {
            // if k is even, the larger middle remainder must be paired with itself as well.
            let count = buckets[k as usize / 2];
            if count % 2 != 0 {
                return false;
            }
        }

        // be careful with the boundary point related to odd and even k
        for i in 1..((k + 1) as usize / 2) {
            if buckets[i] != buckets[k as usize - i] {
                return false;
            }
        }

        return true;
    }
}

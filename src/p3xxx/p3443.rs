// Problem 3443
impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut max = 0;

        for preferred in [b"NE", b"NW", b"SE", b"SW"] {
            let mut used = 0;
            let mut dist = 0;

            for ch in s.bytes() {
                if preferred[0] == ch || preferred[1] == ch {
                    dist += 1;
                } else if used < k {
                    dist += 1;
                    used += 1;
                } else {
                    dist -= 1;
                }
                max = max.max(dist);
            }
        }

        max
    }
}

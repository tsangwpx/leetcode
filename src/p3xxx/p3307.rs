// Problem 3307
impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        /*
         * Originally "a" is at index = 0, length = 1 = 2**0
         * If operations[0] = 1, "a" become "ab", length = 2**1
         * If operations[0] = 0, "a" become "aa", length = 2**1
         *
         * k is 1-based index
         * so 2**(N-1) <= (k-1) < length=2**N
         *
         *  k = 10 - 1 = 9 = 0b1001
         *  [0, 1, 0, 1]
         *
         *
         * i = 0
         * len = 2
         * mutations = 0
         *
         * i = 1
         * len = 4
         * mutations = 1
         *
         * i = 2
         * len = 8
         * mutations = 1
         * a"a"bb aabb
         *
         * i = 3
         * len = 16
         * mutations = 2
         * aabbaabb b"b"ccbbcc
         */

        let k = k - 1;
        let mut mutations = 0;
        let mut len = 1i64;

        for (i, op) in operations.iter().copied().enumerate() {
            // println!("{} {}", i, k & (1 << i));
            if k & (1 << i) != 0 {
                mutations += op;
            }

            if len > k {
                break;
            }

            len <<= 1;
        }

        (b'a' + (mutations % 26) as u8) as char
    }
}

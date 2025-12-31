// Problem 717
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut idx = 0;
        let mut res = None;

        while idx < bits.len() {
            let b = bits[idx];

            if b == 0 {
                idx += 1;
                res = Some(true);
            } else if b == 1 {
                assert!(idx + 1 < bits.len());
                idx += 2;
                res = Some(false);
            } else {
                panic!("bad bit");
            }
        }

        res.unwrap()
    }
}

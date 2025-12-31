// Problem 201
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        // the leading one in XOR is the first difference
        let xor = left ^ right;

        // compute the shift to make the difference disappear
        let mut shift = 0;
        while (xor >> shift) != 0 {
            shift += 1;
        }

        // the reverse shift is the prefix mask
        // left shift may overflow, which is probably undefined behavior in signed integers
        // do right shift, and then left shift to obtain the prefix
        ((i32::MAX >> shift) << shift) & left & right
    }
}

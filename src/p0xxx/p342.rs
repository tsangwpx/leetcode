// Problem 342
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        /**
         * log(2 ** 31) / log (4) = 15.5
         * so 31-bits integer can contains 4 ** 15 but not 4 ** 16
         * we need 16 slots to store 4 ** 0 up to 4 ** 15
         */
        const LEN: usize = 16;
        const PRODUCTS: [i32; LEN] = {
            let mut res = [0i32; LEN];
            res[0] = 1;

            let mut idx = 1;
            while idx < LEN {
                res[idx] = res[idx - 1] * 4;
                idx += 1;
            }
            res
        };

        n >= 1 && PRODUCTS.contains(&n)
    }
}

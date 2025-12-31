// Problem 326
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        /**
         * log(2 ** 31) / log (3) = 19.559
         * so 31-bits integer can contains 3 ** 19 but not 3 ** 20
         * we need 20 slots to store 3 ** 0 up to 3 ** 19
         */
        const LEN: usize = 20;
        const PRODUCTS: [i32; LEN] = {
            let mut res = [1i32; LEN];
            let mut idx = 1;
            while idx < LEN {
                res[idx] = res[idx - 1] * 3;
                idx += 1;
            }
            res
        };

        n >= 1 && PRODUCTS.contains(&n)
    }
}

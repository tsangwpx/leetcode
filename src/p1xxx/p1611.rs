// Problem 1611
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        use std::collections::HashMap;

        let mut memo = HashMap::<i32, i32>::new();
        //

        fn get_msb(n: i32) -> i32 {
            for shift in (0..32).rev() {
                let bit = 1 << shift;

                if bit & n == bit {
                    return shift;
                }
            }

            0
        }

        fn dfs(memo: &mut HashMap<i32, i32>, n: i32) -> i32 {
            if n == 0 || n == 1 {
                return n;
                // } else if n == 2 {
                //     // "10" -> "11" -> "01" -> "00"
                //     return 3;
                // } else if n == 3 {
                //     // "11" -> "01" -> "00"
                //     return 2;
                // } else if n == 4 {
                //     // "100" -> "101" -> "111" -> "110" -> "010" -> "011" -> "001" -> "000"
                //     return 7;
                // } else if n == 9 {
                // "1001"
                // => "1011" => "1010" => "1110" => "1111" => "1101"
                // => "1100" => "0100" => "0101" => "0111" => "0110"
                // => "0010" => "0011" => "0001" => "0000"
            }

            if let Some(op) = memo.get(&n).copied() {
                return op;
            }

            let msb = get_msb(n);
            assert!(msb >= 1);

            let mask = (1 << msb) - 1;

            let op1 = dfs(memo, 1 << (msb - 1));
            let op2 = dfs(memo, mask & n);

            let mut count = 0;

            // operations to make masked bits with a leading one and all zeros followed
            count += op1 - op2;

            // remove the msb
            count += 1;

            // operations to remove all remaining bits
            count += op1;

            memo.insert(n, count);
            // println!("dfs {} mask {} {} count {}", n, op1, op2, count);

            count
        }

        dfs(&mut memo, n)
    }
}

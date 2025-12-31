// Problem 2954
impl Solution {
    const MOD: i64 = 10i64.pow(9) + 7;

    /// Solution
    ///
    /// 1. Split the queue into uninfected partition
    ///
    /// 2. Every second there is one person is newly infected in one of the partiton.
    ///     This forms a multinomial coefficient C = N! / p_1! / p_2! / ... / p_k!
    ///     where p_k are partition size, and N = sum of p_k.
    ///     So, there are C ways to choose partiton such that everyone infected.
    ///
    /// 3. In each partition, there is has W_k = 2.pow(p_k - 1) ways that partiton become
    ///     fully infected. However, there are exceptions: the leftmost and rightmost partiton open
    ///     to the end have only one way to be fully infected. W_left = W_right = 1.
    ///
    /// The final answer is C * W_1 * W_2 * .. * W_k.
    ///
    pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
        assert!(n >= 2);
        assert!(sick.len() >= 1);

        // Find sizes of uninfected partitions
        // The left and right ones which are open to the ends are specials

        let left_unbounded = sick[0]; // size of leftmost unbounded partition
        let right_unbounded = n - *sick.last().unwrap() - 1; // ... of rightmost

        let mut partitions = vec![]; // size of bounded partitions
        let mut part_start = sick[0] + 1; // start index of bounded partitions

        for &idx in sick.iter().skip(1) {
            if part_start != idx {
                // part_start is not infected, make a new partition up to idx (exclusive)
                partitions.push(idx - part_start);
            }
            part_start = idx + 1;
        }

        // compute
        let mut ans = 1;

        for &part_size in partitions.iter() {
            // If the partition size is N + 1
            // the possible ways is W = 2 ** N
            // multiply W to the answer

            ans *= Self::pow_mod(2, part_size as u32 - 1);
            ans %= Self::MOD;
        }

        // compute the multinomial coefficient, C mod M
        // C = N! / p_1! / p_2! / ... / p_k!
        // where k is the partition index, p_k is the k-th partition size and N = sum of p_k

        // include unbounded partition in the coefficient if any
        if left_unbounded > 0 {
            partitions.push(left_unbounded);
        }
        if right_unbounded > 0 {
            partitions.push(right_unbounded);
        }

        // compute the numerator
        // ans *= Self::multi_factorials(&[n - sick.len() as i32], MOD);
        ans *= Self::_fast_fact(n as i64 - sick.len() as i64);
        ans %= Self::MOD;

        // compute the denominator
        let mut denominator = 1i64;
        for &part_size in partitions.iter() {
            denominator *= Self::_fast_fact(part_size as i64);
            denominator %= Self::MOD;
        }

        // use multiplicative inverse
        ans *= Self::mul_inv(denominator);
        ans %= Self::MOD;

        ans as i32
    }

    /// Compute multiplicative inverse with the prime modulo
    #[inline]
    fn mul_inv(num: i64) -> i64 {
        use std::convert::TryFrom;

        if num < 0 {
            panic!("num < 0");
        }

        // Fermat's little theorem because modulo is a large prime
        Self::pow_mod(num, u32::try_from(Self::MOD - 2).expect("modulo too big"))
    }

    /// Compute number.pow(exp) % MOD
    #[inline]
    fn pow_mod(num: i64, mut exp: u32) -> i64 {
        if num < 0 {
            panic!("Negative number");
        } else if num == 0 && exp == 0 {
            panic!("Zero to the power of zero")
        } else if num <= 1 {
            return num;
        }

        let mut res = 1i64;
        let mut square = num;

        loop {
            if (exp & 1) == 1 {
                res *= square;
                res %= Self::MOD;
            }

            exp >>= 1;
            if exp == 0 {
                break;
            }

            square *= square;
            square %= Self::MOD;
        }

        res
    }

    // precomputed factorial table
    const _FACT_STEP: usize = 32;
    const _FACT_SIZE: usize = 3125 + 1;
    const _FACT_TABLE: [i32; Self::_FACT_SIZE] = Self::_fact_table();

    const fn _fact_table() -> [i32; Self::_FACT_SIZE] {
        assert!(
            Self::_FACT_STEP * (Self::_FACT_SIZE + 1) >= 10usize.pow(5),
            "Factorial table do not over 10e5",
        );

        let mut table = [0i32; Self::_FACT_SIZE];
        table[0] = 1;

        let mut factorial: i64 = 1;
        let mut k = 1;
        let mut idx = 1;

        while idx < table.len() {
            let target = idx * Self::_FACT_STEP;

            while k < target {
                k += 1;
                factorial *= k as i64;
                factorial %= Self::MOD;
            }

            table[idx] = factorial as i32;
            idx += 1;
        }

        table
    }

    /// Compute factorial with precomuted table
    #[inline]
    fn _fast_fact(n: i64) -> i64 {
        assert!(n >= 0);
        if n == 0 || n == 1 {
            return 1;
        }

        let idx = n as usize / Self::_FACT_STEP;
        let remaining = n as usize % Self::_FACT_STEP;
        let mut k = (idx * Self::_FACT_STEP) as i64;
        let mut res = Self::_FACT_TABLE[idx] as i64;

        for _ in 0..remaining {
            k += 1;
            res *= k;
            res %= Self::MOD;
        }

        res as i64
    }
}

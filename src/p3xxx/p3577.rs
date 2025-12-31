// Problem 3577
impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        let n = complexity.len();
        if n <= 1 {
            return 1; // 0! or 1! is 1, and for n=1 only computer 0, no unlocking needed
        }

        // Check if complexity[0] is strictly less than all others
        let root = complexity[0];
        for &c in &complexity[1..] {
            if c <= root {
                return 0;
            }
        }

        // All computers 1..n-1 can be unlocked in any order using computer 0
        // So answer is (n-1)!
        let mut fact: i64 = 1;
        for i in 2..n {
            fact = fact * i as i64 % MOD;
        }
        fact as i32
    }
}

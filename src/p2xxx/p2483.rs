// Problem 2483
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        // observations
        // 1. Two prefix sum arrays of penalty, A1: one for open without customers, and A2: other for closed with customer
        // 2. The result is the minimum sum of range query A1[i] - A1[0] and A2[-1] - A[i]
        // 3. A1[0] can be removed because it is always zero.
        // 3b. A2[-1] can also be removed because it is constant to penalty
        // 3c. So the penalty equation is simplified to
        //      A1[i] - A2[i]
        // 4. A1[i] = number of N in customers[0..i]
        //      A2[i] = number of Y in customers[0..i]
        // 5. A1[i] - A2[i] = # of N - (i - # of N)
        //      =  2 * # of N - i
        //      = i - 2 * # of Y
        // note that 2 * # of Y is the profit in the open hours. no surprised. =.=

        let mut earliest = 0;
        let mut penalty = 0;
        let mut ncount = 0;

        for (idx, ch) in customers.bytes().enumerate() {
            let idx = idx + 1;

            if ch == b'N' {
                ncount += 1;
            }
            let p2 = ncount * 2 - idx as i32;

            if p2 < penalty {
                earliest = idx;
                penalty = p2;
            }
        }

        earliest as i32
    }
}

// Problem 781
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        const LIMIT: usize = 1001 + 1;
        let mut counter = [0i32; LIMIT];
        for ans in answers.iter().copied() {
            counter[ans as usize + 1] += 1;
        }
        let mut count = 0;

        for (ans, ans_count) in counter.iter().copied().enumerate().skip(1) {
            let ans = ans as i32;

            // ceil div
            let group = (ans_count + ans - 1) / ans;

            count += group * ans;
        }

        count
    }
}

// Problem 43
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // Naive multiplication
        // without divide-and-conquer
        // Time complexity: O(N * M) where N, M are string lengths

        let num1 = num1
            .into_bytes()
            .into_iter()
            .rev()
            .map(|c| c - b'0')
            .collect::<Vec<_>>();
        let num2 = num2
            .into_bytes()
            .into_iter()
            .rev()
            .map(|c| c - b'0')
            .collect::<Vec<_>>();

        // reserve 2 more digits for carrying
        let mut result = vec![0u8; num1.len() + num2.len() + 2];
        for (idx1, digit1) in num1.iter().copied().enumerate() {
            for (idx2, digit2) in num2.iter().copied().enumerate() {
                let product = digit1 * digit2;
                let mut offset = idx1 + idx2;
                result[offset] += product;

                while result[offset] >= 10 {
                    let (q, r) = (result[offset] / 10, result[offset] % 10);
                    result[offset] = r;
                    result[offset + 1] += q;
                    offset += 1;
                }
            }
        }

        // remove trailing zero
        while result.len() >= 2 && result.last().copied() == Some(0) {
            result.pop();
        }
        result.reverse();
        result.iter_mut().for_each(|c| *c = (*c) + b'0');
        String::from_utf8(result).unwrap()
    }
}

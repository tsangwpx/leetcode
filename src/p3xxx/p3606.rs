// Problem 3606
impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        fn is_valid_code(code: &str) -> bool {
            !code.is_empty()
                && code
                    .bytes()
                    .all(|s| matches!(s, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
        }

        fn is_valid_business_line(business_line: &str) -> Option<usize> {
            ["electronics", "grocery", "pharmacy", "restaurant"]
                .iter()
                .position(|s| *s == business_line)
        }

        let mut indices = code
            .iter()
            .enumerate()
            .filter_map(|(idx, code)| {
                if is_active[idx]
                    && is_valid_code(&code)
                    && is_valid_business_line(&business_line[idx]).is_some()
                {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        indices.sort_by_key(|&idx| {
            (
                is_valid_business_line(&business_line[idx]).unwrap(),
                &code[idx],
            )
        });

        indices
            .into_iter()
            .map(|idx| code[idx].clone())
            .collect::<Vec<_>>()
    }
}

// Problem 2138
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;

        (0..s.len())
            .step_by(k as usize)
            .map(|start| {
                let stop = s.len().min(start + k);
                let mut item = String::with_capacity(k);
                item.push_str(&s[start..stop]);

                for _ in item.len()..k {
                    item.push(fill);
                }

                item
            })
            .collect::<Vec<_>>()
    }
}

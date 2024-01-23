impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::<String>::with_capacity(if digits.is_empty() {
            0
        } else {
            4usize.pow(digits.len() as u32)
        });

        for (step, ch) in digits.chars().enumerate() {
            let letters = match ch {
                '2' => "abc",
                '3' => "def",
                '4' => "ghi",
                '5' => "jkl",
                '6' => "mno",
                '7' => "pqrs",
                '8' => "tuv",
                '9' => "wxyz",
                _ => unreachable!(),
            };

            if step == 0 {
                for ch in letters.chars() {
                    let mut item = String::with_capacity(digits.len());
                    item.push(ch);
                    res.push(item);
                }
            } else {
                for i in 0..res.len() {
                    let len = step;
                    let mut it = letters.chars();
                    res[i].push(it.next().unwrap());

                    for ch in it {
                        let mut item = String::with_capacity(digits.len());
                        item.extend(res[i][0..len].chars());
                        item.push(ch);
                        res.push(item);
                    }
                }
            }
        }

        res
    }
}

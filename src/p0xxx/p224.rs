// Problem 224
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.into_bytes();
        let mut idx = 0;

        let mut stack = Vec::<i32>::with_capacity(16);
        let mut result = 0;
        let mut sign = 1;
        let mut number = 0;

        while idx < s.len() {
            let ch = s[idx];
            idx += 1;

            match ch {
                b'0'..=b'9' => {
                    number = number * 10 + (ch - b'0') as i32;
                }
                b'+' => {
                    // complete the last operation
                    result += sign * number;

                    // start a new operaton with positive value
                    sign = 1;
                    number = 0;
                }
                b'-' => {
                    // complete the last operation
                    result += sign * number;

                    // start a new operaton with negative value
                    sign = -1;
                    number = 0;
                }
                b'(' => {
                    // save current result and the sign of the parentheses to stack
                    stack.push(result);
                    stack.push(sign);

                    // reset the state to default values
                    result = 0;
                    sign = 1;
                    number = 0;
                }
                b')' => {
                    // complete the last operation
                    result += sign * number;

                    // popping saved states
                    result *= stack.pop().unwrap(); // sign of the parentheses
                    result += stack.pop().unwrap(); // prev result

                    number = 0; // reset number
                    // sign is later given by the input "+" or "-"
                }
                _ => {
                    // ignore
                }
            }
        }

        result += sign * number;

        result
    }
}

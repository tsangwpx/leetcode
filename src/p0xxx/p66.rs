// Problem 66
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for value in digits.iter_mut().rev() {
            let new_value = *value + 1;

            if new_value >= 10 {
                *value = 0;
            } else {
                *value = new_value;
                break;
            }
        }

        if digits[0] == 0 {
            digits.insert(0, 1);
        }

        digits
    }
}

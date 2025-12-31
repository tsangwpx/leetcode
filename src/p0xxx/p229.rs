use std::str::pattern::Pattern;

// Problem 229
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut num1 = 0;
        let mut count1 = 0;
        let mut num2 = 1;
        let mut count2 = 0;

        for number in nums.iter().copied() {
            if number == num1 {
                count1 += 1;
            } else if number == num2 {
                count2 += 1;
            } else if count1 == 0 {
                num1 = number;
                count1 = 1;
            } else if count2 == 0 {
                num2 = number;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        let threshold = nums.len() / 3 + 1;
        let (count1, count2) = nums.iter().fold((0, 0), |(c1, c2), &item| {
            if item == num1 {
                (c1 + 1, c2)
            } else if item == num2 {
                (c1, c2 + 1)
            } else {
                (c1, c2)
            }
        });

        let mut res = vec![];
        if count1 >= threshold {
            res.push(num1);
        }

        if count2 >= threshold {
            res.push(num2);
        }

        res
    }
}

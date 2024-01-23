use core::num;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

// Problem 12
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let table = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut num = num;
        let mut roman = String::new();

        for (amount, literal) in table {
            while num >= amount {
                roman.push_str(literal);
                num -= amount;
            }
        }

        roman
    }
}

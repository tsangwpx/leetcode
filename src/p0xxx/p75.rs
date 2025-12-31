// Problem 75
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // following pointer point to last red and blue
        let mut red = 0usize; // exclusive
        let mut white = 0; // exclusive
        let mut blue = nums.len() - 1;

        while white <= blue {
            match nums.get(white).unwrap() {
                0 => {
                    nums.swap(white, red);
                    red += 1;
                    white += 1;
                }
                1 => white += 1,
                2 => {
                    nums.swap(white, blue);
                    if blue == 0 {
                        break;
                    }
                    blue -= 1;
                }
                _ => unreachable!(),
            }
        }
    }
}

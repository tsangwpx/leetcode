use std::iter;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        assert!(height.len() >= 2);
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut area = 0;

        while i < j {
            let left = height[i];
            let right = height[j];
            let width = (j - i) as i32;
            let new_area = width * std::cmp::min(left, right);

            if new_area > area {
                area = new_area;
            }

            if left <= right {
                // Find a higher wall from left
                i += 1;
                while i < j && height[i] < left {
                    i += 1;
                }
            }

            if right <= left {
                // Find a higher wall from right
                // j is initially at least 1
                j -= 1;

                while j > 0 && j > i && height[j] < right {
                    j -= 1;
                }
            }
        }

        area
    }
}

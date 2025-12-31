// Problem 42
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0i32;
        let mut left_wall = 0;
        let mut right_wall = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left <= right {
            left_wall = left_wall.max(height[left]);
            right_wall = right_wall.max(height[right]);

            // println!("{} {}: {} {} {}", left, right, left_wall, right_wall, ans);

            if left == right {
                ans += (left_wall.min(right_wall) - height[left]).max(0);
                break;
            }

            if left_wall <= right_wall {
                ans += (left_wall - height[left]).max(0);
                left += 1;
            }

            if right_wall <= left_wall {
                ans += (right_wall - height[right]).max(0);
                right = right.wrapping_sub(1);
            }
        }
        ans
    }

    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut wall = height.clone();

        let mut top = 0;
        for (idx, base) in height.iter().enumerate() {
            top = top.max(*base);
            wall[idx] = top;
        }

        let mut top = 0;
        for (idx, base) in height.iter().enumerate().rev() {
            top = top.max(*base);
            wall[idx] = wall[idx].min(top);
        }

        // println!("{:?}", wall);
        let mut res = 0;

        for (&top, &base) in wall.iter().zip(height.iter()) {
            res += (top - base).max(0);
        }

        res
    }
}

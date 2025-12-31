// Problem 1306
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        use std::collections::VecDeque;
        let start = start as usize;
        assert!(start < arr.len());
        let mut visited = vec![false; arr.len()];
        visited[start] = true;
        let mut pending = VecDeque::from([start]);

        while let Some(pos) = pending.pop_front() {
            if arr[pos] == 0 {
                return true;
            }
            let left = pos.wrapping_sub(arr[pos] as usize);

            if left < visited.len() && !visited[left] {
                visited[left] = true;
                pending.push_back(left);
            }

            let right = pos.wrapping_add(arr[pos] as usize);

            if right < visited.len() && !visited[right] {
                visited[right] = true;
                pending.push_back(right);
            }
        }

        false
    }
}

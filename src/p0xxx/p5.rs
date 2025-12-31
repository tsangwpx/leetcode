use core::panicking::panic;
use std::cmp::min;
use std::collections::VecDeque;
use std::io::Read;
use std::ops::Range;
use std::str::from_utf8;

impl Solution {
    pub fn longest_palindrome(mut s: String) -> String {
        use std::collections::VecDeque;
        use std::str::from_utf8;

        const LENGTH_MAX: usize = 1001;
        let bytes = s.as_bytes();
        assert!(bytes.len() <= LENGTH_MAX);
        let mut odd_radii = [1u16; LENGTH_MAX];
        let mut evn_radii = [0u16; LENGTH_MAX];
        let mut longest_range = 0..1;

        let mut pos = 1;

        while pos < bytes.len() {
            let mut odd_radius = odd_radii[pos] as usize;

            while pos >= odd_radius
                && pos + odd_radius < bytes.len()
                && bytes[pos - odd_radius] == bytes[pos + odd_radius]
            {
                odd_radius += 1;
            }

            odd_radii[pos] = odd_radius as u16;
            if odd_radius * 2 - 1 > longest_range.len() {
                longest_range = (pos - odd_radius + 1)..(pos + odd_radius);
            }

            let mut evn_radius = evn_radii[pos] as usize;
            while pos >= evn_radius + 1
                && pos + evn_radius < bytes.len()
                && bytes[pos - evn_radius - 1] == bytes[pos + evn_radius]
            {
                evn_radius += 1;
            }
            evn_radii[pos] = evn_radius as u16;
            if evn_radius * 2 > longest_range.len() {
                longest_range = (pos - evn_radius)..(pos + evn_radius);
            }

            let mid = pos;
            // take the closer endpoint (exclusive)
            let stop = min(pos + odd_radius, pos + evn_radius);
            assert!(stop <= bytes.len());
            pos += 1;

            while pos < stop {
                assert!(mid * 2 >= pos);
                let odd_pos = mid * 2 - pos;

                // TODO Do not want to consider edge/special case. Bye.
                // let evn_pos = odd_pos - 1;
            }
        }
        panic!("I am lazy");
    }
    pub fn longest_palindrome2(mut s: String) -> String {
        use std::collections::VecDeque;
        use std::str::from_utf8;
        let bytes = s.as_bytes();
        let mut longest_range = 0..1;
        let mut stack = VecDeque::<usize>::with_capacity(1001);

        for i in 1..bytes.len() {
            let ch = bytes[i];

            for _ in 0..stack.len() {
                let j = stack.pop_front().unwrap();
                if j > 0 && bytes[j - 1] == ch {
                    stack.push_back(j - 1);
                } else if i - j > longest_range.len() {
                    longest_range = j..i;
                }
            }

            stack.push_back(i);
            if ch == bytes[i - 1] {
                stack.push_back(i - 1);
            }
        }

        let i = bytes.len();
        for _ in 0..stack.len() {
            let j = stack.pop_front().unwrap();
            if i - j > longest_range.len() {
                longest_range = j..i;
            }
        }

        // from_utf8(bytes)?.to_string()
        s.truncate(longest_range.end);
        s.replace_range(..longest_range.start, "");
        s
    }
}

fn main() {
    Solution::longest_palindrome("hello world".to_string());
}

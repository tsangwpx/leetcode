mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 71
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut path = path.into_bytes();
        assert!(path.len() >= 1);
        assert!(path[0] == b'/');

        let mut ri = 1;
        let mut wi = 1;

        while ri < path.len() {
            // println!("{} {}", ri, wi);
            // consume more slahes
            while ri < path.len() && path[ri] == b'/' {
                ri += 1;
            }

            let start = ri;
            let part_len = path
                .iter()
                .skip(start)
                .position(|&s| s == b'/')
                .unwrap_or_else(|| path.len() - start);
            ri += part_len;

            if part_len == 0 {
                // trailing slash
                break;
            }

            // Write a slash if there is not one
            if path[wi - 1] != b'/' {
                path[wi] = b'/';
                wi += 1;
            }

            if part_len == 1 && path[start] == b'.' {
                continue;
            } else if part_len == 2 && path[start] == b'.' && path[start + 1] == b'.' {
                // rewind the write index to the slash second to the last one we just written
                wi = &path[..wi - 1].iter().rposition(|&s| s == b'/').unwrap_or(0) + 1;
            } else {
                if start != wi {
                    path.copy_within(start..start + part_len, wi);
                }
                wi += part_len;
            }
        }

        // remove trailing slashes except the leading slash
        while wi >= 2 && path[wi - 1] == b'/' {
            wi -= 1;
        }

        path.truncate(wi);

        // SAFETY: All bytes are English letters, digits, period, slash, and underscore.
        unsafe { String::from_utf8_unchecked(path) }
    }
}

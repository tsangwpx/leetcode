use core::num;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 1436
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashSet;

        let mut sources = HashSet::with_capacity(paths.len());
        let mut destinations = HashSet::with_capacity(paths.len());

        for mut pair in paths {
            destinations.insert(pair.pop().unwrap());
            sources.insert(pair.pop().unwrap());
        }

        for city in destinations {
            if sources.contains(&city) {
                continue;
            }

            return city;
        }

        unreachable!()
    }
}

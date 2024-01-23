use std::cmp::{min, Ordering, Reverse};
use std::collections::BinaryHeap;
use std::process::id;

type RString = Reverse<String>;

impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        use std::cmp::{Ordering, min};

        let mut result = vec![Vec::<String>::with_capacity(4); search_word.len()];
        products.sort_unstable();
        let products = &products[..];

        let mut left = 0;
        let mut right = products.len();

        for (idx, &ch) in search_word.as_bytes().iter().enumerate() {
            left = left + products[left..right].binary_search_by(|s| {
                match s.as_bytes().get(idx) {
                    Some(&ch2) => ch2.cmp(&ch).then(Ordering::Greater),
                    None => Ordering::Less,
                }
            }).unwrap_or_else(|s| s);
            assert!(left <= products.len());
            right = left + products[left..right].binary_search_by(|s| {
                match s.as_bytes().get(idx) {
                    Some(&ch2) => ch2.cmp(&ch).then(Ordering::Less),
                    None => Ordering::Less,
                }
            }).unwrap_or_else(|s| s);
            assert!(right <= products.len());

            result[idx].extend_from_slice(&products[left..min(left + 3, right)]);
        }

        result
    }

    pub fn suggested_products2(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut result = vec![Vec::<String>::with_capacity(4); search_word.len()];
        let bytes = search_word.as_bytes();

        let mut rank = products
            .into_iter()
            .map(|s| Reverse(s))
            .collect::<BinaryHeap<RString>>();

        let mut tmp = BinaryHeap::with_capacity(rank.len());

        for (i, &ch) in bytes.iter().enumerate() {
            let dst = &mut result[i];

            while let Some(top) = rank.pop() {
                if let Some(&ch2) = top.0.as_bytes().get(i) {
                    if ch == ch2 {
                        if dst.len() < 3 {
                            dst.push(top.0.clone());
                        }
                        tmp.push(top);
                    }
                }
            }

            std::mem::swap(&mut rank, &mut tmp);
        }

        result
    }
}

struct Solution {}

fn main() {}

mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 187
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashMap;

        if s.len() <= 10 {
            return vec![];
        }

        #[inline(always)]
        fn encode_nucleotide(base: u8) -> u32 {
            match base {
                b'A' => 0,
                b'C' => 1,
                b'G' => 2,
                b'T' => 3,
                _ => unreachable!(),
            }
        }

        const MASK: u32 = (1 << 20) - 1;

        let mut hash = 0u32;
        for i in 0..9 {
            hash = ((hash << 2) | encode_nucleotide(s.bytes().nth(i).unwrap())) & MASK;
        }

        let mut repeated = HashMap::<u32, Option<String>>::new();

        for i in 9..s.len() {
            hash = ((hash << 2) | encode_nucleotide(s.bytes().nth(i).unwrap())) & MASK;
            repeated
                .entry(hash)
                .and_modify(|seq| {
                    if seq.is_none() {
                        *seq = Some(s[i - 9..i + 1].to_owned());
                    }
                })
                .or_insert(None);
        }

        repeated.into_values().filter_map(|s| s).collect::<Vec<_>>()
    }
}

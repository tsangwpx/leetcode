use std::process::id;

struct Solution {}

#[derive(Debug)]
struct Debugging {
    pos: usize,
    taken: usize,
    need_delete: usize,
    del_count: usize,
    rep_count: usize,
    no_lower: bool,
    no_upper: bool,
    no_digit: bool,
}

// bb
// aaaaaaaaaaaaaaa
// cccccc


// "aaaabbbbccccddeeddeeddeedd"
// "aaaabbbbccccddeeddee"
// "aabbccddeeddeeddeedd"


// "aaaaAAAAAA000000123456"
// "aazaAAzAA00z00123456"
// "aaaaAAAAAA000000123456"
// "aaaaAAAAAA000000123456"
// "aaaaAAAAAA000000123456"
// size 22
// "aa{aa}"
// Debugging { pos: 0, taken: 0, need_delete: 2, del_count: 0, rep_count: 0, no_lower: true, no_upper: true, no_digit: true }
// Debugging { pos: 1, taken: 1, need_delete: 2, del_count: 0, rep_count: 0, no_lower: false, no_upper: true, no_digit: true }
// "AA[A]AA[A]"
// Debugging { pos: 4, taken: 2, need_delete: 0, del_count: 2, rep_count: 0, no_lower: false, no_upper: true, no_digit: true }
// Debugging { pos: 5, taken: 3, need_delete: 0, del_count: 2, rep_count: 0, no_lower: false, no_upper: false, no_digit: true }
// "00[0]00[0]"
// Debugging { pos: 10, taken: 8, need_delete: 0, del_count: 2, rep_count: 2, no_lower: false, no_upper: false, no_digit: true }
// Debugging { pos: 11, taken: 9, need_delete: 0, del_count: 2, rep_count: 2, no_lower: false, no_upper: false, no_digit: false }
// "123456"
// Debugging { pos: 16, taken: 14, need_delete: 0, del_count: 2, rep_count: 4, no_lower: false, no_upper: false, no_digit: false }
// Debugging { pos: 17, taken: 15, need_delete: 0, del_count: 2, rep_count: 4, no_lower: false, no_upper: false, no_digit: false }
// Debugging { pos: 18, taken: 16, need_delete: 0, del_count: 2, rep_count: 4, no_lower: false, no_upper: false, no_digit: false }
// Debugging { pos: 19, taken: 17, need_delete: 0, del_count: 2, rep_count: 4, no_lower: false, no_upper: false, no_digit: false }
// Debugging { pos: 20, taken: 18, need_delete: 0, del_count: 2, rep_count: 4, no_lower: false, no_upper: false, no_digit: false }
// Debugging { pos: 21, taken: 19, need_delete: 0, del_count: 2, rep_count: 4, no_lower: false, no_upper: false, no_digit: false }
// After
// End
// Debugging { pos: 22, taken: 20, need_delete: 0, del_count: 2, rep_count: 4, no_lower: false, no_upper: false, no_digit: false }
//


impl Solution {
    pub fn strong_password_checker3(password: String) -> i32 {
        use std::cmp::{max, min};
        let bytes = password.as_bytes();
        let mut need_delete = bytes.len().saturating_sub(20);
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;

        let mut last = 0;
        let mut pos = 0;
        macro_rules! println {
            ($($rest:tt)*) => {
            }
        }


        // Records the number of repetition groups of the form:
        // `2 + 3m`, `2 + 3p + 1`, and `2 + 3q + 2`
        // [0]: m + p + q
        // [1]: number of (2 + 3p + 1)
        // [2]: number of (2 + 3q + 2)
        let mut groups = [0usize; 3];

        while pos < bytes.len() {
            let ch = bytes[pos];
            pos += 1;

            has_lower = has_lower || ch.is_ascii_lowercase();
            has_upper = has_upper || ch.is_ascii_uppercase();
            has_digit = has_digit || ch.is_ascii_digit();

            if last == ch {
                let fixed = pos;
                while pos < bytes.len() && bytes[pos] == ch {
                    pos += 1;
                }
                let repetitions = pos - fixed;

                // Count m, p, q into groups[0]
                // +1, +2 to its own group[1], group[2]
                groups[0] += repetitions / 3;
                if repetitions % 3 > 0 {
                    groups[repetitions % 3] += 1;
                }
            } else {
                last = ch;
            }
        }

        let missing = 3 - has_upper as i32 - has_lower as i32 - has_digit as i32;
        println!("group={:?}; missing={}, need_delete={}, size={}", groups, missing, need_delete, bytes.len());
        if bytes.len() < 6 {
            let need_insert = 6 - bytes.len();
            return max(missing, need_insert as i32);
        }

        let mut del_count = 0;
        let mut rep_count = 0;

        // Now crop 2+3q+2 and 2+3p+1 groups by deletions
        for i in 1..=2 {
            let units = min(groups[i], need_delete / i);
            let deletions = units * i;

            groups[i] -= units;
            need_delete -= deletions;
            del_count += deletions;
        }
        println!("group={:?}; need_delete={}: {} {}", groups, need_delete, del_count, rep_count);

        if need_delete >= 3 {
            // and delete 3m groups finally
            // assert!(groups[1] == 0 && groups[2] == 0);

            let units = min(groups[0], need_delete / 3);
            let deletions = units * 3;
            groups[0] -= units;
            need_delete -= deletions;
            del_count += deletions;
        }
        println!("group={:?}; need_delete={}: {} {}", groups, need_delete, del_count, rep_count);

        // If need_delete = 0, the length is correct
        // If need_delete != 0, do some random deletion
        del_count += need_delete;
        need_delete = 0;

        // now we have nothing to delete because of length limit
        // fix repeating problem by replacing characters if any
        rep_count += groups[0]; // replace every 3 characters
        rep_count += groups[1]; // replace last character
        rep_count += groups[2]; // replace second character

        max(missing, rep_count as i32) + del_count as i32
    }


    pub fn strong_password_checker(password: String) -> i32 {
        use std::cmp::{max, min};
        let bytes = password.as_bytes();
        let mut no_lower = true;
        let mut no_upper = true;
        let mut no_digit = true;

        let mut pos = 0;
        let mut taken = 0;
        let mut need_delete = bytes.len().saturating_sub(20);

        let mut last = 0u8;
        let mut del_count = 0;
        let mut rep_count = 0;

        const LENGTH_MAX: usize = 20;

        macro_rules! debugging {
            () => {
                Debugging {pos, taken, need_delete, del_count, rep_count, no_lower, no_upper, no_digit, }
            }
        }
        // macro_rules! println {
        //     ($($rest:tt)*) => {
        //     }
        // }

        println!("size {}", bytes.len());

        while pos < bytes.len() && taken < LENGTH_MAX {
            println!("{:?}", debugging!());
            let ch = bytes[pos];
            pos += 1;

            if no_lower && ch.is_ascii_lowercase() {
                no_lower = false;
                taken += 1;
                last = ch;
                continue;
            }

            if no_upper && ch.is_ascii_uppercase() {
                no_upper = false;
                taken += 1;
                last = ch;
                continue;
            }

            if no_digit && ch.is_ascii_digit() {
                no_digit = false;
                taken += 1;
                last = ch;
                continue;
            }

            if ch != last {
                // take different character
                taken += 1;
                last = ch;
                continue;
            }

            // take it
            taken += 1;

            // there are two repeated character now.
            // pos is advanced in the beginning of the loop.
            // a[pos-2] a[pos-1] ?[pos]

            // lookahead to get number of repeated characters
            let mut pivot = pos;
            while pos < bytes.len() && bytes[pos] == ch {
                pos += 1;
            }

            loop {
                let extra = pos - pivot;
                if need_delete >= extra {
                    // let's delete all extra characters
                    // or break the loop if extra = 0
                    need_delete -= extra;
                    del_count += extra;
                    break;
                }

                // number of repetitions after first two before length limit
                let repetitions = min(extra, LENGTH_MAX - taken);
                if repetitions == 0 {
                    // no repetitions for replacements
                    // aa|
                    break;
                } else if repetitions >= 3 {
                    // reduce repetitions by replacement:
                    // aa|aaa.... => aaXaa|....
                    rep_count += 1;
                    pivot += 3;
                    taken += 3;
                } else if repetitions == 1 && need_delete > 0 {
                    // aa|a|, aa|a|aaaaa, or aa|abbbb
                    // delete all repeated characters
                    del_count += extra;
                    need_delete = need_delete.saturating_sub(extra);
                    break;
                } else {
                    // otherwise, repetitions == 2 or deletions are not necessary (repetitions == 1)
                    // aa|aa|, aa|aa|aaaaaaa, or aa|aabb
                    // aa|a|, aa|a|aaaaa or aa|abb
                    // Do replacement here
                    rep_count += 1;
                    taken += repetitions;
                    let extra = extra - repetitions;

                    if extra > 0 {
                        // aa|aa|aaaaaaa
                        // aa|a|aaaaa
                        // bounded by length limit, delete the rest
                        del_count += extra;
                        need_delete = need_delete.saturating_sub(extra);
                    }
                    break;
                }
            }


            /*
             * D prefer deletion than replacement
             * R is replacement
             * aa|
             * aa|D
             * aa|Ra
             * aa|Raa
             * aa|RaaD
             * aa|RaaRa
             * aa|RaaRaa
             * aa|RaaRaaD
             * aa|RaaRaaDa
             */
        }

        println!("After");
        while pos < bytes.len() {
            println!("{:?}", debugging!());
            // now, taken = 20 and pos >= 20
            // only deletion is allowed for string longer than 20

            let ch = bytes[pos];
            pos += 1;

            if no_lower && ch.is_ascii_lowercase() {
                // delete someone before to make room
                no_lower = false;
                del_count += 1;
                continue;
            }

            if no_upper && ch.is_ascii_uppercase() {
                // delete someone before to make room
                no_upper = false;
                del_count += 1;
                continue;
            }

            if no_digit && ch.is_ascii_digit() {
                // delete someone before to make room
                no_digit = false;
                del_count += 1;
                continue;
            }

            // delete it
            del_count += 1;
        }

        println!("End");
        println!("{:?}", debugging!());
        let missing = no_lower as usize + no_upper as usize + no_digit as usize;
        if bytes.len() < 6 {
            // Short string. always do insertions
            max(6 - bytes.len(), missing) as i32
        } else if bytes.len() <= 20 {
            // medium size. replace is the best
            max(missing, rep_count) as i32
        } else {
            // deletion is the best if possible. then replacement.
            (max(missing, rep_count) + del_count) as i32
        }
    }


    pub fn strong_password_checker2(password: String) -> i32 {
        use std::cmp::max;
        let bytes = password.as_bytes();
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;

        let mut rep_count = 0;  // replacement count (every 3 characters, aaaaaaaaaa => aaXaaXaaXa)
        let mut last = 0u8;
        let mut repeated = 1;
        let mut absent = [true; 256];
        let mut population = 0;

        for &ch in bytes {
            population += absent[ch as usize] as i32;
            absent[ch as usize] = false;

            has_lower = has_lower || ch.is_ascii_lowercase();
            has_upper = has_upper || ch.is_ascii_uppercase();
            has_digit = has_digit || ch.is_ascii_digit();

            if last == ch {
                repeated += 1;

                if repeated % 3 == 0 {
                    // every 3 repeated character, replace one
                    rep_count += 1;
                }
            } else {
                last = ch;
                repeated = 1;
            }
        }

        let missing = 3 - has_upper as i32 - has_lower as i32 - has_digit as i32;

        if bytes.len() <= 20 {
            // If not longer than 20, fix missing character by insert/replace
            // Fix repeated characters by replacements
            let add_or_rep = max(missing, 6 - bytes.len() as i32);
            return max(add_or_rep, rep_count);
        }

        let ops = bytes.len() as i32 - 20;

        assert!(1 <= population && population <= 128);

        if population == 1 {
            // only one character available
        }

        // need at least `bytes.len() - 20` deletions and `missing` replacements


        ops
    }
}
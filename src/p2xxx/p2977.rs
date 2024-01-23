mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2977
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        if source == target {
            return 0;
        }

        assert!(source.len() == target.len());
        assert!(original.len() == changed.len() && original.len() == cost.len());

        use std::borrow::Borrow;
        use std::collections::HashMap;
        use std::rc::Rc;

        // Impl Borrow<str> for Rc<String> by wrapping it
        #[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
        struct RcString(Rc<String>);

        impl Into<RcString> for Rc<String> {
            fn into(self) -> RcString {
                RcString(self)
            }
        }

        impl Borrow<str> for RcString {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl Borrow<String> for RcString {
            fn borrow(&self) -> &String {
                &self.0
            }
        }

        impl Borrow<Rc<String>> for RcString {
            fn borrow(&self) -> &Rc<String> {
                &self.0
            }
        }

        let (string_ids, cost_table) = {
            // This inner scope returns two things:
            // 1. A mapping from string to its unique id among strings of the same length.
            // 2. A mapping from string len to a cost table of Floyd-Warshall algorithm
            //      the cost table is accessed by unique string id

            // combine them into 3-tuple
            let items = original
                .into_iter()
                .zip(changed.into_iter())
                .zip(cost.into_iter())
                .map(|((a, b), c)| (Rc::new(a), Rc::new(b), c))
                .collect::<Vec<_>>();

            // count how many different string of the same length
            let mut len_count = HashMap::<usize, usize>::new();

            // assign a unique id to a string of the same length
            let mut string_ids = HashMap::<RcString, usize>::new();

            // assign id to original strings and changed strings
            items
                .iter()
                .map(|(s, _, _)| s.clone())
                .chain(items.iter().map(|(_, s, _)| s.clone()))
                .for_each(|s| {
                    // string length
                    let len = s.len();
                    // string id
                    let sid = string_ids.entry(s.into()).or_insert(usize::MAX);

                    if *sid == usize::MAX {
                        // this is a new string! Bump the len count by one
                        let count = len_count
                            .entry(len)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);

                        // this count can also be used as string id
                        *sid = *count - 1;
                    }
                });

            // allocate memory for Floyd-Warshall algorithm
            let mut cost_table = len_count
                .into_iter()
                .map(|(len, count)| {
                    let mut table = vec![vec![i32::MAX; count]; count];

                    for i in 0..count {
                        table[i][i] = 0;
                    }

                    (len, table)
                })
                .collect::<HashMap<_, _>>();

            // fill the cost table
            for (src, dst, cost) in items.iter() {
                let len = src.len();
                let src_id = string_ids.get(src).copied().unwrap();
                let dst_id = string_ids.get(dst).copied().unwrap();

                let table = cost_table.get_mut(&len).unwrap();
                table[src_id][dst_id] = table[src_id][dst_id].min(*cost);
            }

            for table in cost_table.values_mut() {
                // Floyd-Warshall algorithm
                let count = table.len();

                for k in 0..count {
                    assert!(table[k].len() == count);

                    for i in 0..count {
                        assert!(table[i].len() == count);

                        for j in 0..count {
                            if table[i][k] == i32::MAX || table[k][j] == i32::MAX {
                                continue;
                            }
                            if table[i][j] > table[i][k] + table[k][j] {
                                table[i][j] = table[i][k] + table[k][j]
                            }
                        }
                    }
                }
            }

            (string_ids, cost_table)
        };

        let mut dp = vec![i64::MAX; source.len() + 1];
        dp[0] = 0;

        for (idx, (a, b)) in source.bytes().zip(target.bytes()).enumerate() {
            if a == b {
                // if both are the same, cost is at least no changed
                dp[idx + 1] = dp[idx + 1].min(dp[idx]);
                continue;
            }

            for (&len, table) in cost_table.iter() {
                // range of the first index of the substring
                let start = (idx + 1).saturating_sub(len);
                // stop index (exclusive)
                let stop = (idx + 1).min(source.len().saturating_sub(len - 1));

                for i in start..stop {
                    // this position is bad as a start
                    if dp[i] == i64::MAX {
                        continue;
                    }

                    let Some(src_id) = string_ids.get(&source[i..i + len]).copied() else {
                        continue;
                    };

                    let Some(dst_id) = string_ids.get(&target[i..i + len]).copied() else {
                        continue;
                    };

                    if table[src_id][dst_id] == i32::MAX {
                        // the transition is impossible
                        continue;
                    }

                    // dp[i] is the min cost to up the i-th index (exclusive).
                    // If it is possible to replace source[i..i+len] with target[i..i+len],
                    // dp[i + len] should be updated!
                    let cost = dp[i].saturating_add(i64::from(table[src_id][dst_id]));

                    dp[i + len] = dp[i + len].min(cost);
                }
            }
        }

        if dp[source.len()] == i64::MAX {
            // i64::MAX mean impossible
            -1
        } else {
            dp[source.len()]
        }
    }
}

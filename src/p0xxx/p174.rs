use std::ops::{Add, Sub};

// Problem 174
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        assert!(m >= 1 && n >= 1);

        let mut dp0 = vec![i32::MAX; n];
        let mut dp1 = vec![i32::MAX; n];

        dp0[n - 1] = 0;

        for i in (0..m).rev() {
            dp1[n - 1] = 0.max(dp0[n - 1] - dungeon[i][n - 1]);

            for j in (0..n.saturating_sub(1)).rev() {
                dp1[j] = 0.max(dp1[j + 1].min(dp0[j]) - dungeon[i][j]);
            }

            // println!("{:?}", dp1);

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0[0] + 1
    }

    pub fn calculate_minimum_hp2(dungeon: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Ordering;
        use std::collections::HashMap;

        let m = dungeon.len();
        let n = dungeon[0].len();
        assert!(m >= 1 && n >= 1);

        // hp -> min_hp
        let mut dp0 = vec![HashMap::<i32, i32>::new(); n];
        let mut dp1 = dp0.clone();

        dp0[0].insert(0, 0);

        for i in 0..m {
            assert!(dungeon[i].len() == n);

            for j in 0..n {
                // dp1[j] is now empty

                let delta = dungeon[i][j];

                if delta == 0 {
                    std::mem::swap(&mut dp0[j], &mut dp1[j]);
                } else {
                    for (cur_hp, min_hp) in dp0[j].drain() {
                        let mut cur_hp = cur_hp + delta;
                        let mut min_hp = min_hp;

                        if cur_hp <= 0 {
                            min_hp = min_hp - cur_hp;
                            cur_hp = 0;
                        }

                        dp1[j]
                            .entry(cur_hp)
                            .and_modify(|init| *init = (*init).min(min_hp))
                            .or_insert(min_hp);
                    }
                }

                if j > 0 {
                    let (first, second) = dp1.split_at_mut(j);
                    let prev = first.last().unwrap();
                    let curr = second.first_mut().unwrap();

                    for (&cur_hp, &min_hp) in prev.iter() {
                        let mut cur_hp = cur_hp + delta;
                        let mut min_hp = min_hp;

                        if cur_hp <= 0 {
                            min_hp = min_hp - cur_hp;
                            cur_hp = 0;
                        }

                        curr.entry(cur_hp)
                            .and_modify(|init| *init = (*init).min(min_hp))
                            .or_insert(min_hp);
                    }
                }

                // find the largest possible cur_hp associated with the minimum of min_hp
                let mut best_cur_hp = 0;
                let mut best_min_hp = i32::MAX;

                // println!("sep");
                for (&cur_hp, &min_hp) in dp1[j].iter() {
                    if min_hp < best_min_hp {
                        best_cur_hp = cur_hp;
                        best_min_hp = min_hp;
                    } else if min_hp == best_min_hp {
                        best_cur_hp = best_cur_hp.max(cur_hp);
                    }
                    // println!("{} {} {} {}", cur_hp, min_hp, best_cur_hp, best_min_hp);
                }

                // Keep items with hp no smaller than the hp of the minimum of min_hp
                dp1[j].retain(|&cur_hp, _| cur_hp >= best_cur_hp);
            }

            // let last = dp1.last().unwrap();

            // if last.len() <= 20 {
            //     println!("{}: {:?}", i, last);
            // }
            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0.last().unwrap().values().min().cloned().unwrap_or(0) + 1
    }
}

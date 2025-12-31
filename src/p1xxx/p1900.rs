// Problem 1900
impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        use std::collections::HashMap;

        const N_MAX: i32 = 28;

        let mut memo = HashMap::<i32, i8>::with_capacity(n as usize * 2);

        fn dfs(memo: &mut HashMap<i32, i8>, first: i8, second: i8, rounds: i8, row: i32) {
            let player_count = row.count_ones();

            if player_count == 1 {
                return;
            }

            if memo.contains_key(&row) {
                return;
            };

            let players = (0..N_MAX as i8).fold(
                Vec::with_capacity(player_count as usize),
                |mut players, shift| {
                    if (1 << shift) & row != 0 {
                        players.push(shift);
                    }
                    players
                },
            );
            // println!("rounds {} {}", rounds, row.count_ones());
            let completions = players.len() / 2;

            let base_new_row = if players.len() % 2 == 0 {
                0
            } else {
                1 << players[completions]
            };

            // brute-force cartesian product
            let mut new_rows = vec![base_new_row];

            for i in 0..completions {
                let p1 = players[i];
                let p2 = players[players.len() - 1 - i];

                if p1 == first && p2 == second {
                    memo.insert(row, rounds);
                    return;
                }

                if p1 == first || p1 == second {
                    for item in new_rows.iter_mut() {
                        *item |= 1 << p1;
                    }
                } else if p2 == first || p2 == second {
                    for item in new_rows.iter_mut() {
                        *item |= 1 << p2;
                    }
                } else {
                    let mut new_rows2 = Vec::with_capacity(new_rows.len() * 2);

                    for item in new_rows.iter().copied() {
                        new_rows2.push(item | (1 << p1));
                        new_rows2.push(item | (1 << p2));
                    }

                    std::mem::swap(&mut new_rows, &mut new_rows2);
                }
            }

            for new_row in new_rows {
                dfs(memo, first, second, rounds + 1, new_row);
            }
        }

        let first_player = first_player as i8 - 1;
        let second_player = second_player as i8 - 1;

        dfs(
            &mut memo,
            first_player.min(second_player),
            first_player.max(second_player),
            1,
            (1 << n) - 1,
        );

        let (min, max) = memo
            .values()
            .copied()
            .fold((i32::MAX, i32::MIN), |(min, max), item| {
                (min.min(i32::from(item)), max.max(i32::from(item)))
            });

        vec![min, max]
    }
}

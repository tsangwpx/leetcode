// Problem 2147
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        #[derive(Debug, Clone, Copy)]
        enum State {
            None,
            Left(i64),
            Right(i64, usize),
        }

        let state = corridor
            .bytes()
            .enumerate()
            .fold(State::None, |state, (idx, ch)| {
                if ch == b'S' {
                    match state {
                        State::None => State::Left(1),
                        State::Left(ways) => State::Right(ways, idx),
                        State::Right(ways, prev) => {
                            let len = (idx - prev) as i64;
                            let ways = ways * len % MOD;
                            State::Left(ways)
                        }
                    }
                } else {
                    state
                }
            });

        match state {
            State::None => 0,
            State::Left(_) => 0,
            State::Right(ways, _) => ways as i32,
        }
    }

    // pub fn number_of_ways(corridor: String) -> i32 {
    //     const MOD: i64 = 10i64.pow(9) + 7;

    //     let seat_indices = corridor
    //         .bytes()
    //         .enumerate()
    //         .filter_map(|(idx, ch)| if ch == b'S' { Some(idx) } else { None })
    //         .collect::<Vec<_>>();

    //     if seat_indices.is_empty() || seat_indices.len() % 2 != 0 {
    //         return 0;
    //     }

    //     let mut res = 1i64;

    //     for chunk in seat_indices[1..].chunks_exact(2) {
    //         let len = chunk[1] - chunk[0];
    //         let ways = len as i64;

    //         res *= ways;
    //         res %= MOD;
    //     }

    //     res as i32
    // }
}

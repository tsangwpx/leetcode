// Problem 3320
impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum Element {
            Fire,
            Water,
            Earth,
        }

        impl Element {
            #[inline]
            fn index(self) -> usize {
                match self {
                    Self::Fire => 0,
                    Self::Water => 1,
                    Self::Earth => 2,
                }
            }

            #[inline]
            fn from(ch: char) -> Self {
                match ch {
                    'F' => Element::Fire,
                    'W' => Element::Water,
                    'E' => Element::Earth,
                    _ => unreachable!(),
                }
            }
        }

        #[inline]
        fn compute_rewards(left: Element, right: Element) -> (i16, i16) {
            match (left, right) {
                (Element::Fire, Element::Fire) => (0, 0),
                (Element::Water, Element::Water) => (0, 0),
                (Element::Earth, Element::Earth) => (0, 0),

                (Element::Fire, Element::Earth) => (1, 0),
                (Element::Earth, Element::Fire) => (0, 1),

                (Element::Water, Element::Fire) => (1, 0),
                (Element::Fire, Element::Water) => (0, 1),

                (Element::Earth, Element::Water) => (1, 0),
                (Element::Water, Element::Earth) => (0, 1),
            }
        }

        const MOD: i64 = 10i64.pow(9) + 7;

        let size = s.len() * 2 + 1;
        let draw_index = s.len();

        let mut dp0 = vec![vec![0; size]; 3];
        let mut dp1 = dp0.clone();

        let alice_move = Element::from(s.chars().nth(0).unwrap());
        for bob_move in [Element::Fire, Element::Water, Element::Earth] {
            let (alice_change, bob_change) = compute_rewards(alice_move, bob_move);
            let balance = bob_change - alice_change;
            dp0[bob_move.index()][(draw_index as isize + balance as isize) as usize] = 1;
        }

        for alice_move in s.chars().skip(1) {
            for table in dp1.iter_mut() {
                table.fill(0);
            }

            let alice_move = Element::from(alice_move);

            for bob_move in [Element::Fire, Element::Water, Element::Earth] {
                for prev_move in [Element::Fire, Element::Water, Element::Earth] {
                    if prev_move == bob_move {
                        continue;
                    }

                    let (alice_change, bob_change) = compute_rewards(alice_move, bob_move);
                    let change = bob_change - alice_change;

                    for prev_idx in 1..size - 1 {
                        let idx = (prev_idx as isize + change as isize) as usize;
                        dp1[bob_move.index()][idx] += dp0[prev_move.index()][prev_idx];
                    }
                }
            }

            for idx in 0..3 {
                for (left, right) in dp0[idx].iter_mut().zip(dp1[idx].iter_mut()) {
                    *left = (*right) % MOD;
                    *right = 0;
                }
            }

            // std::mem::swap(&mut dp0, &mut dp1);
        }

        let count = dp0
            .iter()
            .map(|table| table.iter().skip(draw_index + 1).sum::<i64>())
            .sum::<i64>();

        (count % MOD) as i32
    }

    pub fn count_winning_sequences2(s: String) -> i32 {
        use std::collections::HashMap;

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut dp0 = vec![HashMap::<(i16, i16), i64>::new(); 3];
        let mut dp1 = dp0.clone();

        // for table in dp0.iter_mut() {
        //     table.insert((0, 0), 1);
        // }

        #[derive(Debug, Clone, Copy)]
        enum Element {
            Fire,
            Water,
            Earth,
        }

        match s.chars().nth(0).unwrap() {
            'F' => {
                dp0[0].insert((0, 0), 1);
                dp0[1].insert((0, 1), 1);
                dp0[2].insert((1, 0), 1);
            }
            'W' => {
                dp0[0].insert((1, 0), 1);
                dp0[1].insert((0, 0), 1);
                dp0[2].insert((0, 1), 1);
            }
            'E' => {
                dp0[0].insert((0, 1), 1);
                dp0[1].insert((1, 0), 1);
                dp0[2].insert((0, 0), 1);
            }
            _ => unreachable!(),
        }

        for alice_move in s.chars().skip(1) {
            for table in dp1.iter_mut() {
                table.clear();
            }

            let alice_move = match alice_move {
                'F' => Element::Fire,
                'W' => Element::Water,
                'E' => Element::Earth,
                _ => unreachable!(),
            };

            for idx in 0..3usize {
                for (bob_move, idx2) in
                    [(Element::Fire, 0), (Element::Water, 1), (Element::Earth, 2)]
                {
                    if idx == idx2 {
                        continue;
                    }

                    let (alice_change, bob_change) = match (alice_move, bob_move) {
                        (Element::Fire, Element::Fire) => (0, 0),
                        (Element::Water, Element::Water) => (0, 0),
                        (Element::Earth, Element::Earth) => (0, 0),

                        (Element::Fire, Element::Earth) => (1, 0),
                        (Element::Earth, Element::Fire) => (0, 1),

                        (Element::Water, Element::Fire) => (1, 0),
                        (Element::Fire, Element::Water) => (0, 1),

                        (Element::Earth, Element::Water) => (1, 0),
                        (Element::Water, Element::Earth) => (0, 1),
                    };

                    for (&(mut alice_score, mut bob_score), &delta) in dp0[idx].iter() {
                        alice_score += alice_change;
                        bob_score += bob_change;

                        let count = dp1[idx2].entry((alice_score, bob_score)).or_default();

                        *count = (*count + delta) % MOD;
                    }
                }
            }

            println!("{:?}", dp1);

            std::mem::swap(&mut dp0, &mut dp1);
        }

        let count = dp0
            .iter()
            .map(|table| {
                table.iter().fold(0i64, |acc, (&(alice, bob), &count)| {
                    if bob > alice {
                        acc + i64::from(count)
                    } else {
                        acc
                    }
                })
            })
            .sum::<i64>();

        (count % MOD) as i32
    }

    pub fn count_winning_sequences3(s: String) -> i32 {
        use std::collections::HashMap;

        #[derive(Debug, Clone, Copy, PartialEq)]
        enum Element {
            Fire,
            Water,
            Earth,
        }

        impl Element {
            #[inline]
            fn index(self) -> usize {
                match self {
                    Self::Fire => 0,
                    Self::Water => 1,
                    Self::Earth => 2,
                }
            }

            #[inline]
            fn from(ch: char) -> Self {
                match ch {
                    'F' => Element::Fire,
                    'W' => Element::Water,
                    'E' => Element::Earth,
                    _ => unreachable!(),
                }
            }
        }

        #[inline]
        fn compute_rewards(left: Element, right: Element) -> (i16, i16) {
            match (left, right) {
                (Element::Fire, Element::Fire) => (0, 0),
                (Element::Water, Element::Water) => (0, 0),
                (Element::Earth, Element::Earth) => (0, 0),

                (Element::Fire, Element::Earth) => (1, 0),
                (Element::Earth, Element::Fire) => (0, 1),

                (Element::Water, Element::Fire) => (1, 0),
                (Element::Fire, Element::Water) => (0, 1),

                (Element::Earth, Element::Water) => (1, 0),
                (Element::Water, Element::Earth) => (0, 1),
            }
        }

        const MOD: i64 = 10i64.pow(9) + 7;
        const MOVES: [Element; 3] = [Element::Fire, Element::Water, Element::Earth];

        let mut dp0 = vec![HashMap::new(); 3];
        let mut dp1 = dp0.clone();

        let alice_move = Element::from(s.chars().nth(0).unwrap());
        for bob_move in [Element::Fire, Element::Water, Element::Earth] {
            let (alice_change, bob_change) = compute_rewards(alice_move, bob_move);
            let balance = bob_change - alice_change;
            dp0[bob_move.index()].insert(balance, 1i64);
        }

        for alice_move in s.chars().skip(1) {
            let alice_move = Element::from(alice_move);

            for prev_move in MOVES {
                for (balance, count) in dp0[prev_move.index()].drain() {
                    let count = count % MOD;
                    for bob_move in MOVES {
                        if prev_move == bob_move {
                            continue;
                        }

                        let (alice_change, bob_change) = compute_rewards(alice_move, bob_move);
                        let balance = balance + bob_change - alice_change;

                        *dp1[bob_move.index()].entry(balance).or_default() += count;
                    }
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        let count = dp0
            .iter()
            .map(|table| {
                table
                    .iter()
                    .filter_map(|(balance, count)| {
                        if *balance > 0i16 {
                            Some(*count % MOD)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>()
            })
            .sum::<i64>();

        (count % MOD) as i32
    }
}

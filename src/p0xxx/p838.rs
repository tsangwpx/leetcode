// Problem 838
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.into_bytes();

        enum Direction {
            Left,
            Right,
        }

        let mut idx = 0;

        while idx < dominoes.len() {
            if dominoes[idx] != b'.' {
                idx += 1;
                continue;
            }

            let start = idx;

            while idx < dominoes.len() && dominoes[idx] == b'.' {
                idx += 1;
            }

            let start_direction = if start >= 1 && dominoes[start - 1] == b'R' {
                Direction::Right
            } else {
                Direction::Left
            };

            let stop = idx;
            let stop_direction = if stop < dominoes.len() && dominoes[stop] == b'L' {
                Direction::Left
            } else {
                Direction::Right
            };

            match (start_direction, stop_direction) {
                (Direction::Left, Direction::Left) => {
                    dominoes[start..stop].fill(b'L');
                }
                (Direction::Left, Direction::Right) => {}
                (Direction::Right, Direction::Left) => {
                    let len = stop - start;
                    let mid = start + len / 2;
                    dominoes[start..mid].fill(b'R');
                    if len % 2 == 0 {
                        dominoes[mid..stop].fill(b'L');
                    } else {
                        dominoes[mid + 1..stop].fill(b'L');
                    }
                }
                (Direction::Right, Direction::Right) => {
                    dominoes[start..stop].fill(b'R');
                }
            }
        }

        String::from_utf8(dominoes).unwrap()
    }
}

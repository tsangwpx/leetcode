// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 79
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn find_start(board: &Vec<Vec<char>>, word: &mut Vec<u8>) -> Vec<(usize, usize)> {
            let first_byte = *word.first().unwrap();
            let last_byte = *word.last().unwrap();
            let mut first_coords = vec![];
            let mut last_coords = vec![];

            let m = board.len();
            let n = board[0].len();

            for i in 0..m {
                for j in 0..n {
                    let ch = board[i][j] as u8;
                    if ch == first_byte {
                        first_coords.push((i, j));
                    } else if ch == last_byte {
                        last_coords.push((i, j));
                    }
                }
            }

            if first_byte == last_byte || first_coords.len() <= last_coords.len() {
                first_coords
            } else {
                word.reverse();
                last_coords
            }
        }

        fn search_here(
            board: &Vec<Vec<char>>,
            word: &[u8],
            visited: &mut Vec<Vec<bool>>,
            row: usize,
            col: usize,
        ) -> bool {
            if visited[row][col] {
                return false;
            }

            if board[row][col] as u8 != word[0] {
                return false;
            }

            if word.len() == 1 {
                return true;
            }

            let word = &word[1..];
            let m = board.len();
            let n = board[0].len();

            visited[row][col] = true;
            // top, down, left, right
            let ok = false
                || (row > 0 && search_here(board, word, visited, row - 1, col))
                || (row + 1 < m && search_here(board, word, visited, row + 1, col))
                || (col > 0 && search_here(board, word, visited, row, col - 1))
                || (col + 1 < n && search_here(board, word, visited, row, col + 1));
            visited[row][col] = false;

            ok
        }

        let mut word = word.into_bytes();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let coords = find_start(&board, &mut word);

        for (row, col) in coords {
            if search_here(&board, &word, &mut visited, row, col) {
                return true;
            }
        }

        false
    }

    pub fn exist2(board: Vec<Vec<char>>, word: String) -> bool {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let board: Vec<Vec<u8>> = board
            .into_iter()
            .map(|s| s.into_iter().map(|s| s as u8).collect())
            .collect();

        let word = word.into_bytes();
        let m = board.len();
        let n = board[0].len();
        let mut memo = HashSet::<(usize, usize)>::new();

        fn search_here(
            board: &Vec<Vec<u8>>,
            word: &Vec<u8>,
            memo: &mut HashSet<(usize, usize)>,
            row: usize,
            col: usize,
            idx: usize,
        ) -> bool {
            if memo.contains(&(row, col)) {
                return false;
            }

            if board[row][col] != word[idx] {
                return false;
            }

            if idx + 1 == word.len() {
                return true;
            }

            let m = board.len();
            let n = board[0].len();
            memo.insert((row, col));

            if row > 0 && search_here(board, word, memo, row - 1, col, idx + 1) {
                // top
                return true;
            }

            if row + 1 < m && search_here(board, word, memo, row + 1, col, idx + 1) {
                // down
                return true;
            }

            if col > 0 && search_here(board, word, memo, row, col - 1, idx + 1) {
                // left
                return true;
            }

            if col + 1 < n && search_here(board, word, memo, row, col + 1, idx + 1) {
                // right
                return true;
            }
            memo.remove(&(row, col));

            false
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] != word[0] {
                    continue;
                }

                memo.clear();
                if search_here(&board, &word, &mut memo, i, j, 0) {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    // println!("123456");

    use std::hint::black_box;

    let input: Vec<Vec<char>> = Vec::new();
    let input = black_box(input);
    let output = Solution::num_islands(input);
    let output = black_box(output);

    println!("{:?}", output);
    // println!("456789");
}

// Problem 200
impl Solution {
    #[inline(never)]
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        assert!(grid.len() >= 1 && grid.len() <= 300);
        let m = grid.len();
        let n = grid[0].len();
        for row in grid.iter() {
            assert!(row.len() == n && row.len() >= 1 && row.len() <= 300);
        }

        let mut pending = Vec::<(i16, i16)>::with_capacity(m.max(n));
        let mut count = 0i32;

        const WATER: char = '0';
        const ISLAND: char = '1';
        const VISITED: char = 'x';

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != ISLAND {
                    continue;
                }
                count += 1;
                pending.push((i as i16, j as i16));

                while let Some((i, j)) = pending.pop() {
                    if i < 0 || i >= m as i16 || j < 0 || j >= n as i16 {
                        continue;
                    }

                    let cell = &mut grid[i as usize][j as usize];
                    if *cell != ISLAND {
                        continue;
                    }
                    *cell = VISITED;

                    pending.push((i - 1, j));
                    pending.push((i + 1, j));
                    pending.push((i, j - 1));
                    pending.push((i, j + 1));
                }
            }
        }

        count
    }
}

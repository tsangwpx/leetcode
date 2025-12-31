// Problem 417
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        type Ocean = i8;
        const OCEAN_NONE: Ocean = 0;
        const OCEAN_PACIFIC: Ocean = 1 << 0;
        const OCEAN_ATLANTIC: Ocean = 1 << 1;
        const OCEAN_BOTH: Ocean = OCEAN_PACIFIC | OCEAN_ATLANTIC;

        let mut cells = vec![vec![OCEAN_NONE; n]; m];
        let mut pending = vec![];

        #[inline]
        fn add_ocean(cells: &mut Vec<Vec<Ocean>>, i: usize, j: usize, ocean: Ocean) -> bool {
            let old = cells[i][j];
            let new = old | ocean;
            cells[i][j] = new;
            old != new
        }

        // top edge
        for j in 0..n {
            let i = 0;
            if add_ocean(&mut cells, i, j, OCEAN_PACIFIC) {
                pending.push((i, j));
            }
        }

        // left edge
        for i in 0..m {
            let j = 0;
            if add_ocean(&mut cells, i, j, OCEAN_PACIFIC) {
                pending.push((i, j));
            }
        }

        // bottom edge
        for j in 0..n {
            let i = m - 1;
            if add_ocean(&mut cells, i, j, OCEAN_ATLANTIC) {
                pending.push((i, j));
            }
        }

        // right edge
        for i in 0..m {
            let j = n - 1;
            if add_ocean(&mut cells, i, j, OCEAN_ATLANTIC) {
                pending.push((i, j));
            }
        }

        while let Some((i, j)) = pending.pop() {
            let h = heights[i][j];

            for (i2, j2) in [
                (i.wrapping_sub(1), j),
                (i, j.wrapping_sub(1)),
                (i, j.wrapping_add(1)),
                (i.wrapping_add(1), j),
            ] {
                if i2 >= m || j2 >= n {
                    continue;
                }
                let h2 = heights[i2][j2];
                if h2 < h {
                    continue;
                }

                let ocean = cells[i][j];

                if add_ocean(&mut cells, i2, j2, ocean) {
                    pending.push((i2, j2));
                };
            }
        }

        let mut res = vec![];
        for i in 0..m {
            for j in 0..n {
                if cells[i][j] == OCEAN_BOTH {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }

        res
    }
}

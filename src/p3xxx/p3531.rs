// Problem 3531
impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let buildings = buildings
            .into_iter()
            .map(|s| {
                let &[x, y] = s.as_slice() else {
                    panic!("bad position");
                };

                [x, y]
            })
            .collect::<Vec<_>>();

        let n = n as usize;
        let mut xlimits = vec![[n, 0]; n + 1];
        let mut ylimits = vec![[n, 0]; n + 1];

        for &[x, y] in buildings.iter() {
            let x = x as usize;
            let y = y as usize;

            if x <= n && y <= n {
                let [xmin, xmax] = xlimits[y];
                xlimits[y] = [xmin.min(x), xmax.max(x)];

                let [ymin, ymax] = ylimits[x];
                ylimits[x] = [ymin.min(y), ymax.max(y)];
            }
        }

        buildings.iter().fold(0, |acc, &[x, y]| {
            let x = x as usize;
            let y = y as usize;

            if x <= n && y <= n {
                let [xmin, xmax] = xlimits[y];
                let [ymin, ymax] = ylimits[x];

                if xmin < x && x < xmax && ymin < y && y < ymax {
                    return acc + 1;
                }
            }

            acc
        })
    }
}

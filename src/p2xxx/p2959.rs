// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 2959
impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        assert!(n <= 10);

        const INFINITY: i32 = 10i32.pow(6);
        const BAD_INDEX: usize = 10;

        let mut ans = 0;

        let mut table = [BAD_INDEX; 10];
        let mut dists = [[INFINITY; 10]; 10];

        fn branch_exists(mask: u16, shift: u16) -> bool {
            (mask >> shift) & 1 == 1
        }

        'outer: for mask in 0..2u16.pow(n as u32) {
            table.fill(BAD_INDEX);

            let mut nodes = 0usize;

            for shift in 0..n as u16 {
                if branch_exists(mask, shift) {
                    table[shift as usize] = nodes;
                    nodes += 1;
                }
            }
            let nodes = nodes;

            assert!(nodes <= 10);

            dists.fill([INFINITY; 10]);
            for k in 0..nodes {
                dists[k][k] = 0;
            }
            for road in roads.iter() {
                if let [u, v, w] = road[..] {
                    assert!(u >= 0 && u < 10);
                    assert!(v >= 0 && v < 10);

                    if !branch_exists(mask, u as u16) || !branch_exists(mask, v as u16) {
                        continue;
                    }

                    let i = table[u as usize];
                    let j = table[v as usize];

                    let d = w.min(dists[i][j]);
                    dists[i][j] = d;
                    dists[j][i] = d;
                }
            }

            for k in 0..nodes {
                for i in 0..nodes {
                    for j in 0..nodes {
                        let a = dists[i][j];
                        let b = dists[i][k] + dists[k][j];
                        if a > b {
                            dists[i][j] = b;
                            dists[j][i] = b;
                        }
                    }
                }
            }

            for i in 0..nodes {
                for j in 0..nodes {
                    if dists[i][j] > max_distance {
                        continue 'outer;
                    }
                }
            }

            // for i in 0..nodes {
            //     println!(
            //         "{:?}",
            //         (0..nodes)
            //             .into_iter()
            //             .map(|s| dists[i][s])
            //             .collect::<Vec<_>>()
            //     );
            // }

            // println!("{:?}", table);
            ans += 1;
        }

        ans
    }
}

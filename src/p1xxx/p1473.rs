impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        use ::std::cmp::min;

        const M: usize = 101;
        const N: usize = 21;

        let m = m as usize; // number of houses, max number of neighbours
        let n = n as usize; // number of colours
        let target = target as usize;

        println!("m={}, n={}, target={}", m, n, target);
        println!("house: {:?}", houses);
        println!("cost: {:?}", cost);

        assert!(1 <= m && m < M);
        assert!(1 <= n && n < N);
        assert_eq!(houses.len(), m);
        assert_eq!(cost.len(), m);
        assert!(1 <= target && target <= m);

        // dp[colour][neighbour] = cost
        // dp0 is the prev house; dp1 is the working house

        // let mut heap = BinaryHeap::with_capacity(2);

        #[derive(Debug, Clone)]
        struct Fee {
            // Track the two cheapest fee and the cheapest colour
            colour: i32,
            // colour2: i32,
            fee: i32,
            alt: i32,
            records: Vec<i32>,
        }

        impl Fee {
            fn new(n: usize) -> Self {
                Self { colour: 0, fee: i32::MAX, alt: i32::MAX, records: vec![i32::MAX; n] }
            }

            #[inline(always)]
            fn reset(&mut self) {
                self.records.fill(i32::MAX);
                self.colour = 0;
                self.fee = i32::MAX;
                self.alt = i32::MAX;
            }

            #[inline(always)]
            fn update(&mut self, colour: i32, fee: i32) {
                self.records[colour as usize - 1] = fee;

                if fee < self.fee {
                    self.colour = colour;
                    self.alt = self.fee;
                    self.fee = fee;
                } else if fee < self.alt {
                    self.alt = fee;
                }
            }
        }

        // dp[neighbour count]
        let mut dp0 = vec![Fee::new(n); target + 1];
        let mut dp1 = vec![Fee::new(n); target + 1];

        // Precompute the costs of no houses to zeros
        dp1[0].fee = 0;
        dp1[0].alt = 0;

        println!("Init {:?}", dp1);

        for (i, &fixed_colour) in houses.iter().enumerate() {
            std::mem::swap(&mut dp0, &mut dp1);

            for min_fee in dp1.iter_mut() {
                min_fee.reset();
            }

            for j in 0..n {
                let colour = j as i32 + 1;

                if fixed_colour == 0 || colour == fixed_colour {
                    for k in 1..target + 1 {
                        let prev = &dp0[k - 1];
                        let curr = &dp0[k];

                        let mut fee = i32::MAX;
                        if prev.colour == colour {
                            fee = fee.min(prev.alt);
                        } else {
                            fee = fee.min(prev.fee);
                        }

                        fee = fee.min(curr.records[j]);

                        if fixed_colour == 0 {
                            fee = fee.saturating_add(cost[i][j]);
                        }

                        dp1[k].update(colour, fee);
                    }
                } else {
                    // target colour is different from the fixed colour
                    // no neighbour count can be changed
                }
            }

            println!("H{} {}: {:?}", i, fixed_colour, dp1);
        }

        println!("{:?}", dp1[target]);

        let result = &dp1[target];

        match result.fee.min(result.fee) {
            i32::MAX => -1,
            s => s,
        }
    }

    pub fn min_cost3(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        use ::std::cmp::min;

        const M: usize = 101;
        const N: usize = 21;

        let m = m as usize; // number of houses, max number of neighbours
        let n = n as usize; // number of colours
        let target = target as usize;

        assert!(1 <= m && m < M);
        assert!(1 <= n && n < N);
        assert_eq!(houses.len(), m);
        assert_eq!(cost.len(), m);
        assert!(1 <= target && target <= m);

        // dp[colour][neighbour] = cost
        // dp0 is the prev house; dp1 is the working house


        let mut dp0 = vec![vec![i32::MAX; target + 1]; n];
        let mut dp1 = vec![vec![i32::MAX; target + 1]; n];

        // work[colour][neighbour] = min cost excluded colour
        let mut work = vec![vec![i32::MAX; target + 1]; n];

        // Initialize the costs of no houses to zeros
        for j in 0..n {
            dp1[j][0] = 0;
        }

        // println!("Init: {:?}", dp1);

        for (i, &fixed_colour) in houses.iter().enumerate() {
            std::mem::swap(&mut dp0, &mut dp1);

            for j in 0..n {
                let colour = j as i32 + 1;

                if fixed_colour == 0 || colour == fixed_colour {
                    dp1[j][0] = i32::MAX;   // the house must be coloured; neighbours >= 1


                    for k in 1..target + 1 {
                        // Iterate the previous house to find the minimum cost which gives the same neighbour
                        let mut fee = i32::MAX;

                        for prev in 0..n {
                            fee = fee.min(match j == prev {
                                true => dp0[prev][k],   // same colour
                                false => dp0[prev][k - 1],// different colour
                            });
                        }

                        if fixed_colour == 0 {
                            fee = fee.saturating_add(cost[i][j]);
                        }

                        dp1[j][k] = fee;
                    }
                } else {
                    // target colour is different from the fixed colour; set all costs to MAX
                    dp1[j].fill(i32::MAX);
                }
            }

            // println!("H{} {}: {:?}", i, fixed_colour, dp1);
        }

        match (0..n).map(|s| dp1[s][target]).min().unwrap() {
            i32::MAX => -1,
            s => s,
        }
    }

    pub fn min_cost2(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        use ::std::cmp::min;

        let m = m as usize; // number of houses, max number of neighbours
        let n = n as usize; // number of colours
        let target = target as usize;

        assert_eq!(houses.len(), m);
        assert_eq!(cost.len(), m);
        // assert!(1 <= m && m < M);
        // assert!(1 <= n && n < N);
        assert!(1 <= target && target <= m);

        // dp[house][colour][neighbour] = cost
        let mut dp = vec![vec![vec![i32::MAX; m + 1]; n]; m + 1];

        // Initialize the costs of no houses to zeros
        for i in 0..n {
            dp[0][i][0] = 0;
        }

        println!("Init: {:?}", dp[0]);


        for (i, &fixed_colour) in houses.iter().enumerate() {
            for j in 0..n {
                let colour = j as i32 + 1;
                if fixed_colour > 0 && colour != fixed_colour {
                    // colour is fixed; skip other colour
                    continue;
                }

                for k in 1..m + 1 {
                    let mut fee = i32::MAX;
                    for prev in 0..n {
                        fee = fee.min(if j == prev {
                            dp[i][prev][k]
                        } else {
                            dp[i][prev][k - 1]
                        });
                    }

                    if fixed_colour == 0 {
                        fee = fee.saturating_add(cost[i][j]);
                    }
                    dp[i + 1][j][k] = fee;
                }
            }

            println!("H{} {}: {:?}", i, fixed_colour, dp[i + 1]);
        }

        match (0..n).map(|s| dp[m][s][target]).min().unwrap() {
            i32::MAX => -1,
            s => s,
        }
    }
}

struct Solution {}

fn main() {
    Solution::min_cost(
        vec![0, 0, 0, 0, 0],
        vec![vec![1, 10], vec![10, 1], vec![10, 1], vec![1, 10], vec![5, 1]],
        5, 2, 3,
    );
    Solution::min_cost(
        vec![0, 2, 1, 2, 0],
        vec![vec![1, 10], vec![10, 1], vec![10, 1], vec![1, 10], vec![5, 1]],
        5, 2, 3,
    );
    Solution::min_cost(
        vec![3, 1, 2, 3],
        vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
        4, 3, 3,
    );
    println!("Hello World");
}
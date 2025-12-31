// Problem 2902
impl Solution {
    pub fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        use std::collections::HashMap;

        const MOD: i32 = 10i32.pow(9) + 7;

        let mut counter = HashMap::with_capacity(nums.len());

        for number in nums {
            if number > r {
                continue;
            }

            counter
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        assert!(l >= 0 && r >= l);
        let len = r as usize + 1;

        let mut dp = vec![0i32; len];

        // handle zeros and empty set here
        if let Some(count) = counter.remove(&0) {
            dp[0] = count + 1;
        } else {
            dp[0] = 1;
        }

        for (&number, &num_count) in counter.iter() {
            let number = number as usize;
            let num_count = num_count as usize;

            for sum in (0..len).rev() {
                if dp[sum] == 0 {
                    continue;
                }

                let low_bound = sum + number;

                if low_bound >= len {
                    continue;
                }

                let mut new_sum = sum + num_count.min((len - sum) / number) * number;
                if new_sum >= len {
                    new_sum -= number;
                }
                // println!("{} {} {} {}", number, num_count, sum, new_sum);

                loop {
                    dp[new_sum] = (dp[new_sum] + dp[sum]) % MOD;

                    if new_sum == low_bound {
                        break;
                    }

                    new_sum -= number;
                }
            }
            // println!("{} {}", number, num_count);
            // println!("{:?}", dp);
        }

        dp[l as usize..len]
            .iter()
            .fold(0, |total, &count| (total + count) % MOD)
    }

    pub fn count_sub_multisets2(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        use std::collections::HashMap;
        const MOD: i32 = 10i32.pow(9) + 7;

        assert!(l >= 0 && r >= l);

        let mut counter = HashMap::with_capacity(nums.len());

        for number in nums {
            if number > r {
                continue;
            }

            counter
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut dp = HashMap::<i32, i32>::new();
        let mut work = dp.clone();

        // handle zeros and empty set here
        if let Some(count) = counter.remove(&0) {
            dp.insert(0, count + 1);
        } else {
            dp.insert(0, 1);
        }

        for (&number, &num_count) in counter.iter() {
            work.reserve(dp.len());

            'next: for (&sum, &total) in dp.iter() {
                for multiple in 1..=num_count {
                    let new_sum = sum + number * multiple;
                    if new_sum > r {
                        continue 'next;
                    }
                    work.entry(new_sum)
                        .and_modify(|count| *count = (*count + total) % MOD)
                        .or_insert(total);
                }
            }

            // add back work to dp
            for (sum, count) in work.drain() {
                dp.entry(sum)
                    .and_modify(|total| *total = (*total + count) % MOD)
                    .or_insert(count);
            }

            // let mut debug = dp.clone().into_iter().collect::<Vec<_>>();
            // debug.sort_unstable();
            // println!("{:?}", debug);
        }

        dp.into_iter().fold(0, |total, (sum, count)| {
            if l <= sum && sum <= r {
                (total + count) % MOD
            } else {
                total
            }
        })
    }
}

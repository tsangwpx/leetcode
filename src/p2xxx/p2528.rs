// Problem 2528
impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        fn check(stations: &[i32], range: usize, budget: i64, target: i64) -> bool {
            // println!("check target {target} ");
            let mut remaining = budget;
            let mut deltas = vec![0i64; stations.len()];

            let mut power = 0i64;
            let mut right = 0;

            for left in 0..stations.len() {
                power += deltas[left];

                while right < stations.len() && (right - left) <= range {
                    power += stations[right] as i64;

                    let idx2 = right + range + 1;

                    if idx2 < deltas.len() {
                        deltas[idx2] -= stations[right] as i64;
                    }

                    right += 1;
                }

                // println!("station {} power {}", left, power);

                let missing = target - power;
                if missing > 0 {
                    // println!("{} {} {} {} ", left, power, missing, remaining);
                    remaining -= missing;

                    if remaining < 0 {
                        break;
                    }

                    if range > 0 {
                        power += missing;
                        let idx2 = left + range * 2 + 1;
                        if idx2 < deltas.len() {
                            deltas[idx2] -= missing;
                        }
                    }
                }
            }

            remaining >= 0
        }

        let mut left = 0i64;
        let mut right = 10i64.pow(12); // inclusive

        while left < right {
            let target = left + (right - left + 1) / 2;
            if check(&stations, r as usize, k as i64, target) {
                left = target;
            } else {
                right = target - 1;
            }
        }

        left
    }
}

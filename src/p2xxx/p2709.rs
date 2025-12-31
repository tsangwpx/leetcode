// Problem 2709
impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        const PRIMES: [i32; 65] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 91, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313,
        ];

        if nums.len() == 1 {
            // there is no pairs, so it is always possible to traverse between all pairs.
            return true;
        }

        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut unique = HashSet::with_capacity(nums.len());
        let mut nums = nums;

        // dedupe...
        nums.retain(|&s| unique.insert(s));

        if unique.contains(&1) {
            // 1 is unreachable because GCD(x, 1) == 1 by definition
            return false;
        }

        drop(unique);

        let nums = nums;

        if nums.len() == 1 {
            return true;
        }

        // Map prime to some group
        let mut prime_groups = HashMap::<i32, i32>::with_capacity(PRIMES.len());
        // Map group to prime members
        let mut group_members = HashMap::<i32, Vec<i32>>::new();
        // Map group to count of numbrs
        let mut group_numbers = HashMap::<i32, i32>::new();

        let mut factors = Vec::with_capacity(PRIMES.len());

        for i in 0..nums.len() {
            let mut number = nums[i];
            let mut best_group = number;

            for prime in PRIMES.iter().copied() {
                if prime > number {
                    break;
                }

                if number % prime != 0 {
                    continue;
                }

                number /= prime;
                while number > 1 && number % prime == 0 {
                    number /= prime;
                }

                factors.push(prime);
                let group = *prime_groups.entry(prime).or_insert_with(|| {
                    group_members.insert(prime, vec![prime]);
                    prime
                });

                best_group = best_group.min(group);
            }

            if number > 1 {
                let prime = number;

                factors.push(prime);
                let group = *prime_groups.entry(prime).or_insert_with(|| {
                    group_members.insert(prime, vec![prime]);
                    prime
                });

                best_group = best_group.min(group);
            }

            // println!("{}: {} {:?}", nums[i], best_group, factors);

            // Put the number to the group with the smallest index
            *group_numbers.entry(best_group).or_default() += 1;

            for factor in factors.drain(..) {
                if factor == best_group {
                    continue;
                }

                let group = prime_groups.get_mut(&factor).unwrap();
                if *group == best_group {
                    continue;
                }
                let tmp = *group;
                *group = best_group;
                let group = tmp;

                if !group_members.contains_key(&group) {
                    println!("Missing {}", group);
                }

                let members = group_members.remove(&group).unwrap();
                for prime in members.iter().copied() {
                    prime_groups.insert(prime, best_group);
                }

                group_members
                    .get_mut(&best_group)
                    .unwrap()
                    .extend(members.into_iter());

                if let Some(count) = group_numbers.remove(&group) {
                    *group_numbers.get_mut(&best_group).unwrap() += count;
                }
            }
            // println!("{:?}", group_members);
        }

        // println!("{:?}", group_members);

        // If we could traverse all pairs, there is one and only one group contains all numbers
        group_numbers.retain(|_, count| *count >= 1);

        let result = group_numbers
            .into_iter()
            .filter_map(|(_, count)| if count >= 1 { Some(count) } else { None })
            .collect::<Vec<_>>();

        // num_counts.len() == 1 && num_counts[0] == nums.len()
        result.len() == 1 && result[0] as usize == nums.len()
    }
}

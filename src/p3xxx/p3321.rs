// Problem 3321
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        use std::cmp::Reverse;
        use std::collections::BTreeSet;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let k = k as usize;
        let x = x as usize;

        let mut sum = 0i64;
        let mut res = Vec::with_capacity(nums.len() - k as usize + 1);

        let mut counter = HashMap::<i32, i32>::new();
        let mut frequent_numbers = BTreeSet::new();
        let mut infrequent_numbers = BTreeSet::new();

        for (idx, number) in nums.iter().copied().enumerate() {
            if idx >= k {
                let prev_idx = idx - k;
                let prev = nums[prev_idx];
                if prev == number {
                    res.push(sum);
                    continue;
                }
                let prev_count = counter.get_mut(&prev).unwrap();
                if frequent_numbers.remove(&(*prev_count, prev)) {
                    sum -= i64::from(prev) * i64::from(*prev_count);
                } else {
                    infrequent_numbers.remove(&(*prev_count, prev));
                }

                *prev_count -= 1;
                if *prev_count >= 1 {
                    infrequent_numbers.insert((*prev_count, prev));
                }
            }

            let count = counter.entry(number).or_default();
            if frequent_numbers.remove(&(*count, number)) {
                sum -= i64::from(number) * i64::from(*count);
            } else {
                infrequent_numbers.remove(&(*count, number));
            }

            *count += 1;
            infrequent_numbers.insert((*count, number));

            // println!("before {} {} {:?} {:?}", idx, sum, frequent_numbers, infrequent_numbers);

            while frequent_numbers.len() < x {
                let Some((count, number)) = infrequent_numbers.pop_last() else {
                    break;
                };

                frequent_numbers.insert((count, number));
                sum += i64::from(number) * i64::from(count);
            }

            loop {
                let Some((last_count, last_num)) = infrequent_numbers.last().copied() else {
                    break;
                };

                let Some((first_count, first_num)) = frequent_numbers.first().copied() else {
                    break;
                };

                if first_count
                    .cmp(&last_count)
                    .then(first_num.cmp(&last_num))
                    .is_gt()
                {
                    break;
                }

                // swap two pairs
                infrequent_numbers.pop_last();
                infrequent_numbers.insert((first_count, first_num));

                frequent_numbers.pop_first();
                frequent_numbers.insert((last_count, last_num));

                sum -= i64::from(first_num) * i64::from(first_count);
                sum += i64::from(last_num) * i64::from(last_count);
            }

            if idx + 1 >= k {
                // pushing to the result when having seen k items
                res.push(sum);
            }

            // println!(
            //     "{} {} {:?} {:?}",
            //     idx, sum, frequent_numbers, infrequent_numbers
            // );
        }

        res
    }

    pub fn find_x_sum2(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        use std::cmp::Reverse;
        use std::collections::BTreeSet;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let mut counter = HashMap::<i32, i32>::new();
        let mut frequent_numbers = BTreeSet::new();
        let mut infrequent_numbers = BTreeSet::new();

        let mut sum = 0i64;

        let k = k as usize;
        let x = x as usize;

        // initialize counter
        for number in nums.iter().take(k as usize).copied() {
            *counter.entry(number).or_default() += 1;
        }
        for (&number, &count) in counter.iter() {
            infrequent_numbers.insert((count, number));
        }
        while !infrequent_numbers.is_empty() && frequent_numbers.len() < x {
            let (count, number) = infrequent_numbers.pop_last().unwrap();
            frequent_numbers.insert((count, number));
            sum += i64::from(count) * i64::from(number);
        }

        let mut res = Vec::with_capacity(nums.len() - k as usize + 1);
        // println!("{} {:?}", sum, frequent_numbers);
        res.push(sum);

        #[inline]
        fn remove_number(
            frequent_numbers: &mut BTreeSet<(i32, i32)>,
            less_frequent_numbers: &mut BTreeSet<(i32, i32)>,
            sum: &mut i64,
            number: i32,
            count: i32,
        ) {
            let key = (count, number);
            if frequent_numbers.remove(&key) {
                *sum -= i64::from(count) * i64::from(number);
            } else {
                less_frequent_numbers.remove(&key);
            }
        }

        for idx in k..nums.len() {
            let prev = idx - k as usize;
            let prev_num = nums[prev];
            let number = nums[idx];

            if prev_num == number {
                // add and remove the same number
                res.push(sum);
                continue;
            }

            let prev_count = counter.get_mut(&prev_num).unwrap();

            remove_number(
                &mut frequent_numbers,
                &mut infrequent_numbers,
                &mut sum,
                prev_num,
                *prev_count,
            );
            *prev_count -= 1;
            let prev_count = *prev_count;

            let count = counter.entry(number).or_default();
            remove_number(
                &mut frequent_numbers,
                &mut infrequent_numbers,
                &mut sum,
                number,
                *count,
            );
            *count += 1;
            let count = *count;

            if prev_count >= 1 {
                infrequent_numbers.insert((prev_count, prev_num));
            }
            infrequent_numbers.insert((count, number));

            while let Some((count1, num1)) = frequent_numbers.first().copied() {
                let (count2, num2) = infrequent_numbers.last().copied().unwrap();

                if count1 < count2 || (count1 == count2 && num1 < num2) {
                    infrequent_numbers.insert(frequent_numbers.pop_first().unwrap());
                    sum -= i64::from(count1) * i64::from(num1);
                } else {
                    break;
                }
            }

            while !infrequent_numbers.is_empty() && frequent_numbers.len() < x {
                let (count, number) = infrequent_numbers.pop_last().unwrap();
                frequent_numbers.insert((count, number));
                sum += i64::from(count) * i64::from(number);
            }

            // println!("{} {:?}", sum, frequent_numbers);
            res.push(sum);
        }

        res
    }
}

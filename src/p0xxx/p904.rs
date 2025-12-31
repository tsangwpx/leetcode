// Problem 904
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
        struct Basket {
            fruit_type: i32,
            fruit_count: i32,
            last_index: usize,
        }

        let mut baskets = Vec::<Basket>::with_capacity(2);
        let mut left = 0;
        let mut max = 0;

        for (idx, fruit_type) in fruits.iter().copied().enumerate() {
            if baskets.len() >= 1 && baskets[0].fruit_type == fruit_type {
                baskets[0].fruit_count += 1;
                baskets[0].last_index = idx;
                continue;
            } else if baskets.len() >= 2 && baskets[1].fruit_type == fruit_type {
                baskets[1].fruit_count += 1;
                baskets[1].last_index = idx;
                continue;
            }

            if baskets.len() >= 2 {
                // save the max so far
                max = max.max(baskets.iter().map(|s| s.fruit_count).sum());

                while left <= idx {
                    let fruit_type = fruits[left];
                    left += 1;

                    let idx2 = if baskets[0].fruit_type == fruit_type {
                        0
                    } else {
                        1
                    };

                    baskets[idx2].fruit_count -= 1;

                    if baskets[idx2].fruit_count == 0 {
                        baskets.remove(idx2);
                        break;
                    }
                }
            }

            baskets.push(Basket {
                fruit_type,
                fruit_count: 1,
                last_index: idx,
            });
        }

        max = max.max(baskets.iter().map(|s| s.fruit_count).sum());

        max
    }
}

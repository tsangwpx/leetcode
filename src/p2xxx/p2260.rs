// Problem 2260
impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut minimum_picks = usize::MAX;
        let mut last_seen = HashMap::<i32, usize>::with_capacity(cards.len());

        for (idx, card) in cards.into_iter().enumerate() {
            let last_idx = last_seen.entry(card).or_insert(idx);

            if idx != *last_idx {
                minimum_picks = minimum_picks.min(idx + 1 - *last_idx);
                *last_idx = idx;
            }
        }

        if minimum_picks == usize::MAX {
            -1
        } else {
            minimum_picks as i32
        }
    }
}

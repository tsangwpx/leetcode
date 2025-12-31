// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Problem 2968
impl Solution {
    pub fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {
        nums.sort();

        let prefix_sums: Vec<i64> = nums
            .iter()
            .scan(0i64, |acc, x| {
                *acc += *x as i64;
                Some(*acc)
            })
            .collect();

        let mut start = 0;
        let mut stop = 1;
        loop {}

        let mut max_score = 1i32;

        max_score
    }
}

// Problem 2353
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::{HashMap, hash_map::Entry::Occupied};
use std::rc::Rc;

struct FoodRatings {
    // Map food to its rating and cuisine
    food_data: HashMap<Rc<String>, (i32, Rc<String>)>,
    // Map cuisine to food
    rankings: HashMap<Rc<String>, BTreeSet<(Reverse<i32>, Rc<String>)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, mut ratings: Vec<i32>) -> Self {
        assert!(foods.len() >= 1);
        assert!(foods.len() == cuisines.len());
        assert!(foods.len() == ratings.len());

        let mut food_data = HashMap::<Rc<String>, (i32, Rc<String>)>::new();
        let mut rankings = HashMap::<Rc<String>, BTreeSet<(Reverse<i32>, Rc<String>)>>::new();

        foods
            .into_iter()
            .zip(cuisines.into_iter().zip(ratings.into_iter()))
            .for_each(|(food, (cuisine, rating))| {
                let food = Rc::new(food);
                let cuisine = Rc::new(cuisine);

                food_data.insert(food.clone(), (rating, cuisine.clone()));

                rankings
                    .entry(cuisine.clone())
                    .or_insert_with(|| BTreeSet::new())
                    .insert((Reverse(rating), food.clone()));
            });

        Self {
            food_data: food_data,
            rankings,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        match self.food_data.entry(Rc::new(food)) {
            Occupied(mut entry) => {
                let food = entry.key().clone();
                let (rating, ref cuisine) = entry.get_mut();

                if *rating == new_rating {
                    return;
                }

                let prioerity_set = self.rankings.get_mut(cuisine).unwrap();
                prioerity_set.remove(&(Reverse(*rating), food.clone()));

                *rating = new_rating;
                prioerity_set.insert((Reverse(*rating), food));
            }
            _ => unreachable!(),
        };
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let priority_set = self.rankings.get(&Rc::new(cuisine)).unwrap();

        let (_, food) = priority_set.iter().next().unwrap();
        String::clone(&food)
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
fn dummy() {}

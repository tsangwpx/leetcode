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

// Problem 2353
use std::cell::{Cell, RefCell};
use std::collections::{
    BinaryHeap, HashMap,
    hash_map::Entry::{Occupied, Vacant},
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CuisineFood(i32, usize, Rc<String>);

#[derive(Debug)]
struct CuisineFoodRanking {
    stale: Cell<bool>,
    heap: RefCell<BinaryHeap<CuisineFood>>,
}

struct FoodRatings {
    food_map: HashMap<Rc<String>, usize>,
    food_data: Vec<(Rc<String>, Rc<String>, i32)>,
    rankings: HashMap<Rc<String>, CuisineFoodRanking>,
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

        let mut data = vec![];
        data.reserve_exact(foods.len());

        for (idx, (food, cuisine)) in foods.into_iter().zip(cuisines.into_iter()).enumerate() {
            data.push((Rc::new(food), Rc::new(cuisine), ratings[idx]));
        }

        data.sort_by(|(a, ..), (b, ..)| a.cmp(&b).reverse());

        let mut food_map: HashMap<Rc<String>, usize> = HashMap::new();
        let mut rankings = HashMap::new();

        for (idx, (food, cuisine, rating)) in data.iter().enumerate() {
            food_map.insert(food.clone(), idx);

            match rankings.entry(cuisine.clone()) {
                Vacant(entry) => {
                    let mut heap = BinaryHeap::new();
                    heap.push(CuisineFood(*rating, idx, food.clone()));

                    entry.insert(CuisineFoodRanking {
                        stale: Cell::new(false),
                        heap: RefCell::new(heap),
                    });
                }
                Occupied(entry) => {
                    let mut heap = RefCell::borrow_mut(&entry.get().heap);
                    heap.push(CuisineFood(*rating, idx, food.clone()));
                }
            }
        }

        // println!("data: {:?}", data);

        Self {
            food_map,
            food_data: data,
            rankings: rankings,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let idx = *self.food_map.get(&Rc::new(food)).unwrap();
        let (food, cuisine, rating) = &mut self.food_data[idx];

        if new_rating != *rating {
            *rating = new_rating;
            let ranking = self.rankings.get(cuisine).unwrap();
            let mut heap = RefCell::borrow_mut(&ranking.heap);

            // println!("change: {:?}", heap);

            let mut peek = heap.peek_mut().unwrap();
            if &peek.2 == food {
                // same food, change the peek only
                peek.0 = new_rating;
            } else {
                drop(peek);

                heap.push(CuisineFood(new_rating, idx, food.clone()));
                ranking.stale.set(true);
            }
        }
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let ranking = self.rankings.get(&Rc::new(cuisine)).unwrap();
        let mut heap = RefCell::borrow_mut(&ranking.heap);

        // println!("high: {:?}", heap);

        if ranking.stale.get() {
            loop {
                let &CuisineFood(rating, idx, _) = heap.peek().unwrap();

                if self.food_data[idx].2 == rating {
                    break;
                }

                heap.pop();
            }
        }

        let CuisineFood(_, _, food) = heap.peek().unwrap();

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

// Problem 2391
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut paper = 0;
        let mut glass = 0;
        let mut metal = 0;
        let mut units = 0;
        for (i, waste) in garbage.iter().enumerate() {
            units += waste.len();

            if waste.contains('P') {
                paper = i;
            }

            if waste.contains('G') {
                glass = i;
            }

            if waste.contains('M') {
                metal = i;
            }
        }

        let mut total_time = units as i32;

        for (i, &time) in travel.iter().enumerate() {
            let house = i + 1;

            if paper >= house {
                total_time += time;
            }
            if glass >= house {
                total_time += time;
            }
            if metal >= house {
                total_time += time;
            }
        }

        total_time
    }
}
